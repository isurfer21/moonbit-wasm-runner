# moonbit-wasm-runner

A lightweight CLI tool to run **MoonBit‑generated WebAssembly modules** using [Wasmtime](https://wasmtime.dev).  
It supports interactive prompts, custom function invocation, argument passing (integers, floats, strings), and verbose logging for debugging.

---

## ✨ Features
- Run `.wasm` files compiled from MoonBit or other sources.
- Provide the Wasm file path via CLI or interactively when omitted.
- Call a specific exported function with `--func` (default: `_start`).
- Pass arbitrary arguments with `--args` (supports `i32`, `f64`, and strings).
- Verbose logging with `--verbose` for debugging and inspection.
- Built on **Wasmtime v39** and **clap v4** for modern Rust ergonomics.

---

## 📦 Installation

Clone the repository and build:

```bash
git clone https://github.com/isurfer21/moonbit-wasm-runner.git
cd moonbit-wasm-runner
cargo build --release
```

The binary will be available at:

```
target/release/moonbit-wasm-runner
```

---

## 🚀 Usage

### Syntax
```bash
moonbit-wasm-runner <wasm_file_path> [OPTIONS]
```

### Options
| Flag / Option | Description |
|---------------|-------------|
| `<wasm_file_path>` | Path to the `.wasm` file to execute. If omitted, you’ll be prompted interactively. |
| `-f, --func <name>` | Name of the exported function to run (default: `_start`). |
| `-a, --args <list>` | Comma‑separated list of arguments (supports integers, floats, and strings). |
| `-v, --verbose` | Enable verbose logging (shows args, results, and execution details). |
| `-h, --help` | Show help menu. |
| `-V, --version` | Show version. |

---

## 🧑‍💻 Examples

### Run default `_start` function
```bash
moonbit-wasm-runner ./moonbit/target/wasm/release/build/main/main.wasm
```

### Run a specific function
```bash
moonbit-wasm-runner ./main.wasm --func add
```

### Pass integer arguments
```bash
moonbit-wasm-runner ./main.wasm --func add --args 5,10
```

### Pass floats
```bash
moonbit-wasm-runner ./main.wasm --func scale --args 3.14,2.71
```

### Pass strings
```bash
moonbit-wasm-runner ./main.wasm --func concat --args hello,world
```

### Verbose mode
```bash
moonbit-wasm-runner ./main.wasm --func add --args 42,7 --verbose
```

---

## 🛠 Development Notes

- **Arguments parsing**:  
  - Integers → `Val::I32`  
  - Floats → `Val::F64` (bit‑encoded)  
  - Strings → `Val::ExternRef`  

- **Dynamic calling**: Uses `Func::call` with runtime type inspection, so you’re not locked into fixed signatures.

- **Extensibility**:  
  - Add more type parsers (e.g., `i64`, `f32`).  
  - Support explicit type annotations (`i32:5,f64:3.14,string:hello`).  
  - Extend logging with execution time or memory usage.

---

## 📂 Project Structure
```
moonbit-wasm-runner/
├── src/
│   └── main.rs        # CLI + Wasmtime runner
├── Cargo.toml         # Dependencies (clap, wasmtime)
└── README.md          # Documentation
```

---

## ⚡ Dependencies
- [clap v4](https://docs.rs/clap/latest/clap/) — modern CLI argument parsing.
- [wasmtime v39](https://docs.rs/wasmtime/latest/wasmtime/) — WebAssembly runtime.

---

## 🧩 Future Improvements
- Add support for WASI imports (filesystem, stdout, etc.).
- Provide JSON output mode for structured results.
- Add benchmarking mode (`--bench`) to measure execution time.
- Integrate with MoonBit’s Wasm Component Model for modular builds.

---

## 📜 License
MIT License. See [LICENSE](LICENSE) for details.
