# CommP comparison across machines

MiB/s at the largest payload each machine ran (`node compare/run.js report` regenerates this file).

| Machine | CPU | SHA ext | Size | `rust` | $\color{#DEA584}{\texttt{rust-simd}}$ | `rust-soft` | `rust-avx512` | `rust-avx2` | `rust-sse2` | `go` | `go-1core` | `go-stdlib` | `go-nosha` | `go-nosha-1core` | `go-generic` | `wasm` | `rust-portable` |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 11th-gen-intel-core-i7-11800h-linux-x64 | 11th Gen Intel(R) Core(TM) i7-11800H @ 2.30GHz | yes | 1 GiB | 494 (1.0c) | $\color{#DEA584}{\textbf{1406 (1.0c)}}$ | 724 (1.0c) | 750 (1.0c) | 440 (1.0c) | 206 (1.0c) | 2237 (2.6c) | 1085 (1.0c) | 505 (2.5c) | 174 (2.3c) | 100 (1.0c) | 103 (2.2c) | 211 (1.0c) |  |
| apple-m1-max-darwin-arm64 | Apple M1 Max (8P + 2E) | yes | 1 GiB | 670 (1.0c) | $\color{#DEA584}{\textbf{703 (1.0c)}}$ | 287 (1.0c) |  |  |  | 1359 (2.6c) | 679 (1.0c) | 835 (2.7c) | 94 (2.1c) | 47 (1.0c) |  | 180 (1.0c) |  |
| apple-m1-pro-darwin-arm64 | Apple M1 Pro (8P + 2E) | yes | 1 GiB | 681 (1.0c) |  | 294 (1.0c) |  |  |  | 728 (2.6c) | 343 (1.0c) | 867 (2.7c) | 136 (2.2c) | 69 (1.0c) |  | 183 (1.0c) | 611 (1.0c) |
| neoverse-v2-linux-arm64 | Neoverse-V2 | yes | 1 GiB | 769 (1.0c) | $\color{#DEA584}{\textbf{859 (1.0c)}}$ | 203 (1.0c) |  |  |  | 1767 (2.4c) | 762 (1.0c) | 576 (2.4c) | 92 (2.1c) | 45 (1.0c) |  | 119 (1.0c) |  |
