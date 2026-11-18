//! 4-lane SHA-256 for wasm32 SIMD
//!
//! sha2's `wasm32_simd128` backend only vectorizes the message schedule; the
//! rounds of each compression stay scalar. Every leaf and every node on a tree
//! level is independent, so here 4 messages are hashed at once, one per `u32`
//! lane of a `v128`, by rust-commp-simd.

use super::{truncated_hash_64, NODE_SIZE};

/// Hash 4 independent 64-byte messages into truncated nodes
#[inline(always)]
fn hash4(msgs: &[[u8; 64]; 4], out: &mut [[u8; NODE_SIZE]; 4]) {
    *out = rust_commp_simd::hash_4_blocks(msgs);
}

/// Hash many 64-byte messages into truncated nodes, 4 at a time
pub fn hash_many(msgs: &[[u8; 64]], out: &mut [[u8; NODE_SIZE]]) {
    let mut msg_groups = msgs.chunks_exact(4);
    let mut out_groups = out[..msgs.len()].chunks_exact_mut(4);
    for (m, o) in (&mut msg_groups).zip(&mut out_groups) {
        hash4(m.try_into().unwrap(), o.try_into().unwrap());
    }
    // A lone message is cheaper scalar; 2 or 3 share one padded group,
    // hashed by recursing so `hash4` is only inlined into the loop above
    match (msg_groups.remainder(), out_groups.into_remainder()) {
        ([], _) => {}
        ([msg], [node]) => *node = truncated_hash_64(msg),
        (rest, nodes) => {
            let mut group = [[0u8; 64]; 4];
            let mut out = [[0u8; NODE_SIZE]; 4];
            group[..rest.len()].copy_from_slice(rest);
            hash_many(&group, &mut out);
            nodes.copy_from_slice(&out[..nodes.len()]);
        }
    }
}
