mod codegens;
mod parsers;
mod utils;

use std::env;
use std::fs;
use codegens::{cpp_codegen, rs_codegen};
use parsers::{rs_parser, cpp_parser};
use utils::{
    prepare_cpp_output_paths, prepare_cpp_template_paths,
    prepare_rust_output_paths, prepare_rust_template_paths,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse CLI args
    let from = args.iter().find(|arg| arg.starts_with("--from=")).map(|arg| arg.trim_start_matches("--from="));
    let to = args.iter().find(|arg| arg.starts_with("--to=")).map(|arg| arg.trim_start_matches("--to="));
    let input = args.iter().find(|arg| arg.starts_with("--input=")).map(|arg| arg.trim_start_matches("--input="));
    let module = args.iter().find(|arg| arg.starts_with("--module=")).map(|arg| arg.trim_start_matches("--module="));

    // Validate all arguments
    if from.is_none() || to.is_none() || input.is_none() || module.is_none() {
        eprintln!(
            "Usage: {} --from=<input_lang> --to=<output_lang> --input=<from_file> --module=<module_name>",
            args[0]
        );
        std::process::exit(1);
    }

    let from = from.unwrap();
    let to = to.unwrap();
    let input = input.unwrap();
    let module = module.unwrap();

    match (from, to) {
        ("rust", "cpp") => {
            let (h_template_path, cpp_template_path, makefile_template_path, demo_template_path) = prepare_cpp_template_paths();

            let rust_source = fs::read_to_string(&input)
                .unwrap_or_else(|err| panic!("Failed to read Rust source file '{}': {}", input, err));
            let functions = rs_parser::parse_exported_functions(&rust_source);

            let h_bindings = cpp_codegen::generate_header(&module, &functions, h_template_path.to_str().unwrap());
            let cpp_bindings = cpp_codegen::generate_cpp(&module, &functions, cpp_template_path.to_str().unwrap());
            let makefile = cpp_codegen::generate_makefile(&module, makefile_template_path.to_str().unwrap());
            let demo = cpp_codegen::generate_demo(&module, demo_template_path.to_str().unwrap());

            let (h_path, cpp_path, mf_path, demo_path) = prepare_cpp_output_paths(module);
            fs::write(&h_path, h_bindings).expect(&format!("Failed to write {}", h_path.display()));
            fs::write(&cpp_path, cpp_bindings).expect(&format!("Failed to write {}", cpp_path.display()));
            fs::write(&mf_path, makefile).expect(&format!("Failed to write {}", mf_path.display()));
            fs::write(&demo_path, demo).expect(&format!("Failed to write {}", demo_path.display()));

            println!("Generated: {}, {}, {}, {}", h_path.display(), cpp_path.display(), mf_path.display(), demo_path.display());
        }

        ("cpp", "rust") => {
            let (rs_template_path, cargo_template_path) = prepare_rust_template_paths();

            let cpp_source = fs::read_to_string(&input)
                .unwrap_or_else(|err| panic!("Failed to read C++ source file '{}': {}", input, err));
            let functions = cpp_parser::parse_exported_functions(&cpp_source);
            println!("Parsed functions: {:?}", functions);

            let rust_bindings = rs_codegen::generate_bindings(&module, &functions, rs_template_path.to_str().unwrap());
            let cargo_toml = rs_codegen::generate_cargo(&module, cargo_template_path.to_str().unwrap());

            let (bindings_path, cargo_path) = prepare_rust_output_paths(module);
            fs::write(&bindings_path, rust_bindings).expect(&format!("Failed to write {}", bindings_path.display()));
            fs::write(&cargo_path, cargo_toml).expect(&format!("Failed to write {}", cargo_path.display()));

            println!("Generated: {}, {}", bindings_path.display(), cargo_path.display());
        }
        _ => {
            eprintln!("Unsupported conversion. Supported: --from=rust --to=cpp");
            std::process::exit(1);
        }
    }
}
