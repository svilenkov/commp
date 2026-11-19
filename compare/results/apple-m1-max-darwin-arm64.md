# apple-m1-max-darwin-arm64

2026-11-19T12:29:41.769Z

|  |  |
| --- | --- |
| CPU | Apple M1 Max (8P + 2E) |
| Cores | 10 physical, 10 logical |
| OS | Darwin 23.6.0 (darwin/arm64) |
| SHA-related CPU features | sha2 sha512 ~~asimd~~ ~~sve~~ ~~sve2~~ |
| Virtualized | no |
| Source | 6e79462fc0 + uncommitted changes (25bcbeb5b44e) |
| Versions | commp 0.1.0, cpufeatures 0.3.1, sha2 0.11.0 / 0.10.9, rust-commp-simd 0.1.0, filecoin-project/go-fil-commp-hashhash v0.0.0-20261118170028-575243346f65 / v0.4.0, minio/sha256-simd v1.0.1 / => ./sha256stdlib, @hugomrdias/commp-wasm 0.1.4, inline wasm sha256 15f91babb6c2cbc6 |
| Toolchains | go version go1.25.0 darwin/arm64, rustc 1.102.0-nightly (9c5648833 2026-11-10), node v26.13.1 |

## Correctness

- Lotus vectors up to 64 MiB: 1072 checks, 0 failures (45 larger vectors skipped)
- Sweep up to 4 MiB: 300 payloads, 0 disagreements
- Variants: rust, rust-simd, rust-par, rust-soft, go, go-stdlib, go-nosha, wasm

## Speed

MiB/s, median of the runs; CPU cores = (user + sys) / wall at 1 GiB.

| Variant | SHA-256 backend | 32 MiB MiB/s | 1 GiB MiB/s | CPU cores |
| --- | --- | --- | --- | --- |
| `rust` | ARMv8 SHA2, 4 messages interleaved | 672.6 | 669.5 | 1.00 |
| `rust-simd` | rust-commp-simd: reduce neon_sha256_x4, extrude neon | 700.7 | 703.2 | 1.00 |
| `rust-par` | ARMv8 SHA2, 4 messages interleaved, 10 threads | 3244.9 | 3553.3 | 8.14 |
| `rust-soft` | NEON, 8 messages per SHA-256 (2 vectors), forced | 287.7 | 286.5 | 1.00 |
| `go` | sha256-simd/ARMv8 | 1360.2 | 1359.4 | 2.61 |
| `go-1core` | sha256-simd/ARMv8 | 672.5 | 678.9 | 1.00 |
| `go-stdlib` | crypto/sha256/ARMv8 | 824.4 | 834.7 | 2.72 |
| `go-nosha` | crypto/sha256/generic | 93.8 | 93.8 | 2.15 |
| `go-nosha-1core` | crypto/sha256/generic | 47.4 | 47.2 | 1.00 |
| `wasm` | wasm simd128, 4 messages per SHA-256 | 180.4 | 179.9 | 1.00 |

## Memory

Peak RSS of the whole process (Node alone is ~40 MiB); growth is peak RSS after hashing minus before. Heap is what the hasher allocated during the run (not tracked for WASM).

| Variant | Payload | Peak RSS | RSS growth | Heap allocated | Allocations |
| --- | --- | --- | --- | --- | --- |
| `rust` | 1 MiB | 1.7 MiB | 176 KiB | 16 KiB | 2 |
| `rust-simd` | 1 MiB | 1.7 MiB | 48 KiB | 128.5 KiB | 1 |
| `rust-par` | 1 MiB | 2.6 MiB | 192 KiB | 16 KiB | 2 |
| `go` | 1 MiB | 7 MiB | 720 KiB | 1.3 MiB | 240 |
| `go-1core` | 1 MiB | 5.5 MiB | 80 KiB | 1.3 MiB | 207 |
| `wasm` | 1 MiB | 64.3 MiB | 704 KiB | n/a | n/a |
| `rust` | 1 GiB | 1.7 MiB | 192 KiB | 16 KiB | 2 |
| `rust-simd` | 1 GiB | 1.9 MiB | 304 KiB | 128.5 KiB | 1 |
| `rust-par` | 1 GiB | 2.9 MiB | 272 KiB | 30.9 KiB | 12 |
| `go` | 1 GiB | 10.8 MiB | 880 KiB | 1290.2 MiB | 33450 |
| `go-1core` | 1 GiB | 32.4 MiB | 18.9 MiB | 1290.2 MiB | 33219 |
| `wasm` | 1 GiB | 66.3 MiB | 3.8 MiB | n/a | n/a |
