# moonbit-wasm-runner

A flexible CLI tool to run **MoonBit‑generated WebAssembly modules** using [Wasmtime](https://wasmtime.dev).  
It supports interactive prompts, custom function invocation, argument passing with **type annotations or autodetection**, and verbose logging for debugging.

---

## ✨ Features
- Run `.wasm` files compiled from MoonBit or other sources.
- Provide the Wasm file path via CLI or interactively when omitted.
- Call a specific exported function with `--func` (default: `_start`).
- Pass arguments with `--args`:
  - **Explicit type annotations**: `i32:5,f64:3.14,string:hello,bool:true`
  - **Autodetection**: `5,3.14,hello,true,false`
- Boolean support:
  - Autodetection: `"true"` / `"false"` → booleans; `"1"` / `"0"` → integers
  - Explicit: `bool:true`, `bool:false`, `bool:1`, `bool:0`
- Verbose logging with `--verbose` for detailed execution info.

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
| `<wasm_file_path>` | Path to the `.wasm` file to execute. If omitted, prompts interactively. |
| `-f, --func <name>` | Name of the exported function to run (default: `_start`). |
| `-a, --args <list>` | Comma‑separated list of arguments. Supports explicit type annotations or autodetection. |
| `-v, --verbose` | Enable verbose logging. |
| `-h, --help` | Show help menu. |
| `-V, --version` | Show version. |

---

## 🧑‍💻 Examples

### Run default `_start` function
```bash
moonbit-wasm-runner ./main.wasm
```

### Run a specific function
```bash
moonbit-wasm-runner ./main.wasm --func add
```

### Pass integers
```bash
moonbit-wasm-runner ./main.wasm --func add --args 5,10
moonbit-wasm-runner ./main.wasm --func add --args i32:5,i32:10
```

### Pass floats
```bash
moonbit-wasm-runner ./main.wasm --func scale --args 3.14,2.71
moonbit-wasm-runner ./main.wasm --func scale --args f64:3.14,f64:2.71
```

### Pass strings
```bash
moonbit-wasm-runner ./main.wasm --func concat --args hello,world
moonbit-wasm-runner ./main.wasm --func concat --args string:hello,string:world
```

### Pass booleans
```bash
moonbit-wasm-runner ./main.wasm --func toggle --args true,false
moonbit-wasm-runner ./main.wasm --func toggle --args bool:true,bool:false
```

### Verbose mode
```bash
moonbit-wasm-runner ./main.wasm --func add --args 42,7 --verbose
```

---

## 📖 Supported Types

| Annotation | Example | Autodetection |
|------------|---------|---------------|
| `i32`      | `i32:5` | `5` |
| `i64`      | `i64:42` | — |
| `f32`      | `f32:3.14` | — |
| `f64`      | `f64:2.71` | `3.14` |
| `string`   | `string:hello` | `hello` |
| `bool`     | `bool:true`, `bool:false`, `bool:1`, `bool:0` | `true`, `false` |

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
