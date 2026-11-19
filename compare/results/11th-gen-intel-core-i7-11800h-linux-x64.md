# 11th-gen-intel-core-i7-11800h-linux-x64

2026-11-19T12:15:34.621Z

|  |  |
| --- | --- |
| CPU | 11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz |
| Cores | 8 physical, 16 logical |
| OS | Linux 6.15.0-rc7 (linux/x64) |
| SHA-related CPU features | sha_ni avx avx2 bmi2 ssse3 sse4_1 avx512f |
| Virtualized | unknown |
| CPU governor | powersave |
| Source | unknown |
| Versions | commp 0.1.0, cpufeatures 0.3.1, sha2 0.11.0 / 0.10.9, rust-commp-simd 0.1.0, filecoin-project/go-fil-commp-hashhash v0.0.0-20261118170028-575243346f65 / v0.4.0, minio/sha256-simd v1.0.1 / => ./sha256stdlib, @hugomrdias/commp-wasm 0.1.4, inline wasm sha256 15f91babb6c2cbc6 |
| Toolchains | go version go1.26.0 linux/amd64, rustc 1.93.0 (254b59607 2026-01-19), node v24.21.0 |

## Correctness

- Lotus vectors up to 64 MiB: 1608 checks, 0 failures (45 larger vectors skipped)
- Sweep up to 4 MiB: 300 payloads, 0 disagreements
- Variants: rust, rust-simd, rust-par, rust-soft, rust-avx512, rust-avx2, rust-sse2, go, go-stdlib, go-nosha, go-generic, wasm

## Speed

MiB/s, median of the runs; CPU cores = (user + sys) / wall at 1 GiB.

| Variant | SHA-256 backend | 32 MiB MiB/s | 1 GiB MiB/s | CPU cores |
| --- | --- | --- | --- | --- |
| `rust` | SHA-NI, 2 messages interleaved | 494.0 | 494.0 | 1.00 |
| `rust-simd` | rust-commp-simd: reduce avx512_x16_soa, extrude avx512_vbmi2 | 1403.7 | 1406.5 | 1.00 |
| `rust-par` | SHA-NI, 2 messages interleaved, 16 threads | 3079.5 | 3090.9 | 13.77 |
| `rust-soft` | AVX-512, 16 messages per SHA-256, forced | 723.0 | 724.0 | 1.00 |
| `rust-avx512` | AVX-512, 16 messages per SHA-256, forced | 749.9 | 750.3 | 1.00 |
| `rust-avx2` | AVX2, 8 messages per SHA-256, forced | 440.7 | 440.2 | 1.00 |
| `rust-sse2` | SSE2, 8 messages per SHA-256 (2 vectors), forced | 205.6 | 205.7 | 1.00 |
| `go` | sha256-simd/SHA-NI | 2242.1 | 2236.7 | 2.61 |
| `go-1core` | sha256-simd/SHA-NI | 1094.6 | 1084.6 | 1.00 |
| `go-stdlib` | crypto/sha256/SHA-NI | 585.7 | 504.8 | 2.51 |
| `go-nosha` | crypto/sha256/AVX2 | 175.2 | 173.8 | 2.28 |
| `go-nosha-1core` | crypto/sha256/AVX2 | 99.9 | 99.8 | 1.00 |
| `go-generic` | crypto/sha256/generic | 104.3 | 102.8 | 2.21 |
| `wasm` | wasm simd128, 4 messages per SHA-256 | 210.9 | 211.0 | 1.00 |

## Memory

Peak RSS of the whole process (Node alone is ~40 MiB); growth is peak RSS after hashing minus before. Heap is what the hasher allocated during the run (not tracked for WASM).

| Variant | Payload | Peak RSS | RSS growth | Heap allocated | Allocations |
| --- | --- | --- | --- | --- | --- |
| `rust` | 1 MiB | 11.9 MiB | 0 B | 16 KiB | 2 |
| `rust-simd` | 1 MiB | 11.9 MiB | 0 B | 128.5 KiB | 1 |
| `rust-par` | 1 MiB | 11.9 MiB | 0 B | 16 KiB | 2 |
| `go` | 1 MiB | 11.9 MiB | 0 B | 1.3 MiB | 236 |
| `go-1core` | 1 MiB | 11.9 MiB | 0 B | 1.3 MiB | 207 |
| `wasm` | 1 MiB | 59.3 MiB | 256 KiB | n/a | n/a |
| `rust` | 1 GiB | 11.9 MiB | 0 B | 16 KiB | 2 |
| `rust-simd` | 1 GiB | 11.9 MiB | 0 B | 128.5 KiB | 1 |
| `rust-par` | 1 GiB | 11.9 MiB | 0 B | 30.9 KiB | 12 |
| `go` | 1 GiB | 14.9 MiB | 3 MiB | 1290.2 MiB | 33610 |
| `go-1core` | 1 GiB | 32.3 MiB | 20.4 MiB | 1290.2 MiB | 33219 |
| `wasm` | 1 GiB | 60.9 MiB | 1.7 MiB | n/a | n/a |
