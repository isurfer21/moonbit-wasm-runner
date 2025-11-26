use clap::{Arg, ArgAction, Command};
use std::io::{self, Write};
use wasmtime::*;
use wasmtime::component::__internal::anyhow;

fn main() -> anyhow::Result<()> {
    // Define CLI interface
    let matches = Command::new("moonbit_wasm_runner")
        .version("2.1")
        .author("Abhishek Kumar <abhishek.kumar.thakur@zohomail.in>")
        .about("Run a Moonbit's WebAssembly module using Wasmtime")
        .arg(
            Arg::new("wasm")
                .help("Path to the .wasm file to execute")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("func")
                .short('f')
                .long("func")
                .help("Name of the exported function to run (default: _start)")
                .num_args(1),
        )
        .arg(
            Arg::new("args")
                .short('a')
                .long("args")
                .help("Comma-separated list of arguments (int, float, or string)")
                .num_args(1),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Resolve wasm path (CLI or prompt)
    let wasm_path = if let Some(path) = matches.get_one::<String>("wasm") {
        path.clone()
    } else {
        print!("Enter path to wasm file: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    };

    // Function name (default: _start)
    let func_name = matches.get_one::<String>("func").map(|s| s.as_str()).unwrap_or("_start");

    // Step 1: collect raw args as strings first (store needed later for externrefs)
    let raw_args: Vec<String> = matches
        .get_one::<String>("args")
        .map(|s| s.split(',').map(|x| x.trim().to_string()).collect())
        .unwrap_or_default();

    let verbose = matches.get_flag("verbose");
    if verbose {
        println!("[verbose] Using wasm file: {}", wasm_path);
        println!("[verbose] Function to run: {}", func_name);
        println!("[verbose] Raw arguments: {:?}", raw_args);
    }

    // Wasmtime config and store
    let mut config = wasmtime::Config::new();
    config.wasm_gc(true);
    config.wasm_function_references(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, ());

    // Step 2: convert raw args into Val after store exists
    let args: Vec<Val> = raw_args
        .iter()
        .map(|x| {
            if let Ok(i) = x.parse::<i32>() {
                Val::I32(i)
            } else if let Ok(f) = x.parse::<f64>() {
                Val::F64(f.to_bits())
            } else {
                Val::ExternRef(Some(
                    ExternRef::new(&mut store, x.clone()).expect("failed to create externref")
                ))
            }
        })
        .collect();

    if verbose {
        println!("[verbose] Parsed arguments (Val): {:?}", args);
    }

    // Load module
    let module = Module::from_file(&engine, &wasm_path)?;

    // Linker and host function
    let mut linker = Linker::new(&engine);
    let print_char = Func::wrap(&mut store, move |c: i32| {
        print!("{}", char::from_u32(c as u32).unwrap_or('?'));
    });
    linker.define(&mut store, "spectest", "print_char", print_char)?;

    // Instantiate
    let instance = linker.instantiate(&mut store, &module)?;

    // Lookup function dynamically
    let run = instance
        .get_func(&mut store, func_name)
        .ok_or_else(|| anyhow::anyhow!("Function '{}' not found", func_name))?;

    // Prepare results buffer based on function type
    let ty = run.ty(&store);
    let mut results = vec![Val::I32(0); ty.results().len()];

    // Call
    run.call(&mut store, &args, &mut results)?;

    if verbose {
        println!("[verbose] Results: {:?}", results);
    }

    Ok(())
}
