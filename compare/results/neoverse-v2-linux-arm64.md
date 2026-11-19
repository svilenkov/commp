# neoverse-v2-linux-arm64

2026-11-19T12:22:41.845Z

|  |  |
| --- | --- |
| CPU | Neoverse-V2 |
| Cores | 4 physical, 4 logical |
| OS | Linux 6.12.74+deb13+1-cloud-arm64 (linux/arm64) |
| SHA-related CPU features | sha2 sha512 asimd sve sve2 |
| Virtualized | unknown |
| Source | unknown |
| Versions | commp 0.1.0, cpufeatures 0.3.1, sha2 0.11.0 / 0.10.9, rust-commp-simd 0.1.0, filecoin-project/go-fil-commp-hashhash v0.0.0-20261118170028-575243346f65 / v0.4.0, minio/sha256-simd v1.0.1 / => ./sha256stdlib, @hugomrdias/commp-wasm 0.1.4, inline wasm sha256 15f91babb6c2cbc6 |
| Toolchains | go version go1.25.0 linux/arm64, rustc 1.94.1 (e408947bf 2026-03-25), node v24.21.0 |

## Correctness

- Lotus vectors up to 64 MiB: 1072 checks, 0 failures (45 larger vectors skipped)
- Sweep up to 4 MiB: 300 payloads, 0 disagreements
- Variants: rust, rust-simd, rust-par, rust-soft, go, go-stdlib, go-nosha, wasm

## Speed

MiB/s, median of the runs; CPU cores = (user + sys) / wall at 1 GiB.

| Variant | SHA-256 backend | 32 MiB MiB/s | 1 GiB MiB/s | CPU cores |
| --- | --- | --- | --- | --- |
| `rust` | ARMv8 SHA2, 4 messages interleaved | 770.7 | 769.0 | 1.00 |
| `rust-simd` | rust-commp-simd: reduce neon_sha256_x4, extrude neon | 854.0 | 858.7 | 1.00 |
| `rust-par` | ARMv8 SHA2, 4 messages interleaved, 4 threads | 2802.7 | 2731.6 | 3.71 |
| `rust-soft` | NEON, 8 messages per SHA-256 (2 vectors), forced | 203.1 | 203.1 | 1.00 |
| `go` | sha256-simd/ARMv8 | 1748.5 | 1767.3 | 2.42 |
| `go-1core` | sha256-simd/ARMv8 | 768.2 | 761.5 | 1.00 |
| `go-stdlib` | crypto/sha256/ARMv8 | 573.2 | 575.9 | 2.40 |
| `go-nosha` | crypto/sha256/generic | 92.0 | 92.1 | 2.15 |
| `go-nosha-1core` | crypto/sha256/generic | 45.3 | 45.3 | 1.00 |
| `wasm` | wasm simd128, 4 messages per SHA-256 | 118.9 | 118.9 | 1.00 |

## Memory

Peak RSS of the whole process (Node alone is ~40 MiB); growth is peak RSS after hashing minus before. Heap is what the hasher allocated during the run (not tracked for WASM).

| Variant | Payload | Peak RSS | RSS growth | Heap allocated | Allocations |
| --- | --- | --- | --- | --- | --- |
| `rust` | 1 MiB | 8.1 MiB | 0 B | 16 KiB | 2 |
| `rust-simd` | 1 MiB | 8.1 MiB | 0 B | 128.5 KiB | 1 |
| `rust-par` | 1 MiB | 8.1 MiB | 0 B | 16 KiB | 2 |
| `go` | 1 MiB | 8.1 MiB | 0 B | 1.3 MiB | 235 |
| `go-1core` | 1 MiB | 8.1 MiB | 0 B | 1.3 MiB | 207 |
| `wasm` | 1 MiB | 57.1 MiB | 456 KiB | n/a | n/a |
| `rust` | 1 GiB | 8.1 MiB | 0 B | 16 KiB | 2 |
| `rust-simd` | 1 GiB | 8.1 MiB | 0 B | 128.5 KiB | 1 |
| `rust-par` | 1 GiB | 8.1 MiB | 0 B | 30.9 KiB | 12 |
| `go` | 1 GiB | 16.6 MiB | 6.1 MiB | 1290.2 MiB | 33311 |
| `go-1core` | 1 GiB | 32.8 MiB | 20.2 MiB | 1290.2 MiB | 33219 |
| `wasm` | 1 GiB | 58.9 MiB | 2 MiB | n/a | n/a |
