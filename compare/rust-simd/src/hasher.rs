//! Streaming CommP over rust-commp-simd's two kernels
//!
//! Input collects in a chunk of CHUNK_QUADS quads. A full chunk is FR32
//! padded in place (extrude_quads) and reduced in place to one node
//! (pairwise_reduce_buffer, all levels in one call while the chunk is in
//! cache). Chunk roots go on a pending stack like a binary counter.
//! At the end the partial chunk is zero filled, reduced and pushed, and the
//! stack folds up with zero commitments to the padded piece size.

use rust_commp_simd::{extrude_quads, features, pairwise_reduce_buffer, sha256_trunc254};

const NODE_SIZE: usize = 32;
const IN_PER_QUAD: usize = 127;
const OUT_PER_QUAD: usize = 128;

/// 1024 quads: 127 KiB in, 128 KiB padded, 4096 nodes, root at level 12
const CHUNK_QUADS: usize = 1024;
const CHUNK_LEVEL: usize = 12;
const CHUNK_IN: usize = CHUNK_QUADS * IN_PER_QUAD;
const CHUNK_OUT: usize = CHUNK_QUADS * OUT_PER_QUAD;

/// The x86-64 x16 kernel writes 512 bytes past the active results
const HEADROOM: usize = 512;

/// Same limit as rs/commp
pub const MAX_PAYLOAD_SIZE: u64 = (IN_PER_QUAD as u64) << 47;
const MIN_PAYLOAD_SIZE: u64 = 65;

const MAX_LEVELS: usize = 64;

pub struct SimdHasher {
    /// Input bytes in buf[..fill], padded in place to CHUNK_OUT
    buf: Vec<u8>,
    fill: usize,
    written: u64,
    /// Full chunks pushed so far
    chunks: u64,
    /// stack[j] is a pending node at level CHUNK_LEVEL + j
    stack: [[u8; NODE_SIZE]; MAX_LEVELS],
    simd_reduce: bool,
}

impl SimdHasher {
    pub fn new() -> Self {
        SimdHasher {
            buf: vec![0u8; CHUNK_OUT + HEADROOM],
            fill: 0,
            written: 0,
            chunks: 0,
            stack: [[0u8; NODE_SIZE]; MAX_LEVELS],
            simd_reduce: features::best_reduce() != "scalar",
        }
    }

    pub fn write(&mut self, mut bytes: &[u8]) -> Result<(), ()> {
        if self.written.saturating_add(bytes.len() as u64) > MAX_PAYLOAD_SIZE {
            return Err(());
        }
        self.written += bytes.len() as u64;
        while !bytes.is_empty() {
            let n = (CHUNK_IN - self.fill).min(bytes.len());
            self.buf[self.fill..self.fill + n].copy_from_slice(&bytes[..n]);
            self.fill += n;
            bytes = &bytes[n..];
            if self.fill == CHUNK_IN {
                let root = self.reduce_chunk(CHUNK_QUADS, CHUNK_QUADS * 4);
                self.push(root);
                self.fill = 0;
            }
        }
        Ok(())
    }

    /// Root and padded piece size
    pub fn finish(mut self) -> Result<([u8; NODE_SIZE], u64), &'static str> {
        if self.written < MIN_PAYLOAD_SIZE {
            return Err("payload too small");
        }
        let tail_quads = self.fill.div_ceil(IN_PER_QUAD);

        // all input in the first chunk: one tree, zero padded to a power of 2
        if self.chunks == 0 {
            let nodes = (tail_quads * 4).next_power_of_two();
            let root = self.reduce_chunk(tail_quads, nodes);
            return Ok((root, (nodes * NODE_SIZE) as u64));
        }

        // a partial last chunk is zero filled to a whole one
        if tail_quads > 0 {
            let root = self.reduce_chunk(tail_quads, CHUNK_QUADS * 4);
            self.push(root);
        }

        let count = self.chunks;
        let height = (count.next_power_of_two().trailing_zeros()) as usize;
        let padded = (NODE_SIZE as u64) << (CHUNK_LEVEL + height);
        if count.is_power_of_two() {
            return Ok((self.stack[height], padded));
        }

        // carry the right-most partial subtree up, zero commitments on the right
        let zero = zero_comms();
        let mut acc: Option<[u8; NODE_SIZE]> = None;
        for j in 0..height {
            let level = CHUNK_LEVEL + j;
            if count >> j & 1 == 1 {
                let right = acc.unwrap_or(zero[level]);
                acc = Some(hash_pair(&self.stack[j], &right));
            } else if let Some(a) = acc {
                acc = Some(hash_pair(&a, &zero[level]));
            }
        }
        Ok((acc.expect("count not a power of 2 has a set bit"), padded))
    }

    /// buf[..fill] holds the input of `quads` quads (the last one may be
    /// partial). Zero fills the rest, pads to `quads` x 128 bytes in place,
    /// zero fills up to `nodes` nodes and reduces them to one root.
    fn reduce_chunk(&mut self, quads: usize, nodes: usize) -> [u8; NODE_SIZE] {
        let len = nodes * NODE_SIZE;
        // the arm64 extrude kernel (2x NEON) takes an even count only, an
        // extra zero quad pads to zero nodes, the same as the zero fill below
        let quads = quads.next_multiple_of(2);
        // the kernel reads 128 bytes of the last quad, 1 past its payload
        let end = (quads * IN_PER_QUAD + 1).min(self.buf.len());
        self.buf[self.fill..end].fill(0);
        unsafe { extrude(self.buf.as_mut_ptr(), quads as u64) };
        if quads * OUT_PER_QUAD < len {
            self.buf[quads * OUT_PER_QUAD..len].fill(0);
        }

        if self.simd_reduce {
            // len is 128 x 2^N: nodes is a power of 2, at least 4
            unsafe { pairwise_reduce_buffer(self.buf.as_mut_ptr(), len, 0) };
        } else {
            let mut pairs = len / 64;
            loop {
                unsafe { rust_commp_simd::pairwise_reduce_buffer_rust_scalar(self.buf.as_mut_ptr(), pairs) };
                if pairs == 1 {
                    break;
                }
                pairs /= 2;
            }
        }
        self.buf[..NODE_SIZE].try_into().unwrap()
    }

    /// Push a chunk root at CHUNK_LEVEL, merging completed subtrees upward
    fn push(&mut self, mut node: [u8; NODE_SIZE]) {
        let mut j = 0;
        let mut c = self.chunks;
        while c & 1 == 1 {
            node = hash_pair(&self.stack[j], &node);
            c >>= 1;
            j += 1;
        }
        self.stack[j] = node;
        self.chunks += 1;
    }
}

/// rust-commp-simd's arm64 extrude kernel uses v8-v15 without saving
/// d8-d15 (callee-saved in AAPCS64), so the call marks them clobbered and
/// the compiler keeps nothing there across it
#[cfg(target_arch = "aarch64")]
unsafe fn extrude(buf: *mut u8, count: u64) {
    core::arch::asm!(
        "bl {f}",
        f = sym extrude_quads,
        inout("x0") buf => _,
        inout("x1") count => _,
        out("v8") _, out("v9") _, out("v10") _, out("v11") _,
        out("v12") _, out("v13") _, out("v14") _, out("v15") _,
        clobber_abi("C"),
    );
}

#[cfg(not(target_arch = "aarch64"))]
unsafe fn extrude(buf: *mut u8, count: u64) {
    extrude_quads(buf, count)
}

fn hash_pair(left: &[u8; NODE_SIZE], right: &[u8; NODE_SIZE]) -> [u8; NODE_SIZE] {
    let mut pair = [0u8; 64];
    pair[..NODE_SIZE].copy_from_slice(left);
    pair[NODE_SIZE..].copy_from_slice(right);
    sha256_trunc254(&pair)
}

/// zero[k] is the root of a tree of 2^k zero nodes
fn zero_comms() -> [[u8; NODE_SIZE]; MAX_LEVELS] {
    let mut zero = [[0u8; NODE_SIZE]; MAX_LEVELS];
    for k in 1..MAX_LEVELS {
        zero[k] = hash_pair(&zero[k - 1], &zero[k - 1]);
    }
    zero
}

/// Kernels this machine picked, for `info`
pub fn backend() -> String {
    format!(
        "rust-commp-simd: reduce {}, extrude {}",
        features::best_reduce(),
        features::best_extrude()
    )
}
