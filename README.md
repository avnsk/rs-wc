# rswc - A High-Performance Rust Clone of the `wc` Utility

A fast, memory-efficient command-line utility built in Rust that counts lines, words, characters, and bytes in a file or from standard input (`stdin`). 

This project was built to solve John Crickett's [Coding Challenges: Build Your Own wc Tool](https://codingchallenges.fyi/challenges/challenge-wc).



### Phase 1: The Non-Buffered Approach (Naive)
Initially, the utility read entire files directly into memory (`Vec<u8>`) all at once. 
* **The Flaw**: While simple, this approach introduces severe bottlenecks when handling large files (e.g., a 10 GB log payload or a compressed database dump). It forces heap allocations proportional to the file size, making it prone to **Out Of Memory (OOM)** runtime crashes.

### Phase 2: Optimized 16KB Fixed-Chunk Streaming (Current)
To safely handle arbitrarily large files, the current codebase streams file inputs through a **16 KB stack-allocated byte buffer (`[0u8; 16384]`)** using an `io::BufReader`.
* **Zero Runtime Heap Allocations**: The memory footprint remains completely fixed at 16 KB regardless of whether you process a 10 KB source file or a 100 GB stream.
* **CPU Cache Alignment**: Reusing a fixed array ensures the data resides consistently inside the CPU's high-speed L1/L2 cache, bypassing slow system RAM fetches.
* **Trait-Based Polymorphism**: Leverages a unified `Box<dyn Read>` wrapper to process physical files and `stdin` streams interchangeably within the same code layout.

---

## CLI Arguments

If no flags are specified, the utility defaults to enabling all counters (`-c`, `-l`, `-w`, and `-m`).


| Flag | Argument | Description |
| :--- | :--- | :--- |
| `-c` | `--bytes` | Counts the total number of bytes. |
| `-l` | `--lines` | Counts the number of newline characters (`\n`). |
| `-w` | `--words` | Counts words separated by whitespace. |
| `-m` | `--chars` | Counts characters using safe UTF-8 multibyte boundary filtering. |

---

## Quick Start & Usage Examples

### Prerequisites
Make sure you have the Rust toolchain installed. If not, fetch it via [rustup.rs](https://rustup.rs).

```bash
git clone <your-repository-url>
cd rswc
```

### Passing File Paths
Run the utility against a local target file by supplying the path after a double dash (`--`):

```bash
# Run with all default counts
cargo run -- "test.txt"

# Count only lines and words
cargo run -- -l -w "test.txt"

# Count bytes and UTF-8 characters
cargo run -- -c -m "test.txt"
```

### Piping via Standard Input (`stdin`)
The utility dynamically falls back to processing `stdin` streams if no file argument is specified:

```bash
# Pipe the output of another command into rswc
cat test.txt | cargo run

# Explicitly test stdin with specific flags
echo "Hello World from Rust" | cargo run -- -w
```

---

## Technical Implementation Highlights

* **UTF-8 Code Point Filtering**: Instead of parsing complete strings (which triggers allocation checks), the `-m` character count isolates multibyte markers by ignoring UTF-8 continuation bytes (`(byte & 0xC0) == 0x80`).
* **CLI Construction**: Uses `clap` (v4 Parser derive architecture) for automated input compilation, error handling, and generated help screens.
