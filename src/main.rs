mod codegen;
mod parser;

use std::env;
use std::fs;
use std::path::PathBuf;

fn prepare_template_paths() -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let templ_dir_path = PathBuf::from("./src/templates");
    if !templ_dir_path.exists() {
        panic!("Templates directory not found");
    }
    let header_templ_path = templ_dir_path.join("cpp_header.hbs");
    let source_templ_path = templ_dir_path.join("cpp_source.hbs");
    let makefile_templ_path = templ_dir_path.join("cpp_makefile.hbs");
    let demo_templ_path = templ_dir_path.join("cpp_demo.hbs");

    (header_templ_path, source_templ_path, makefile_templ_path, demo_templ_path)
}

fn prepare_output_paths(module_name: &str) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let dir_path = PathBuf::from(format!("{}-bindings", module_name));
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path).expect("Failed to create output directory");
    }
    let header_path = dir_path.join(format!("{}.hpp", module_name));
    let cpp_path = dir_path.join(format!("{}.cpp", module_name));
    let makefile_path = dir_path.join("Makefile");
    let demo_path = dir_path.join("demo.cpp");

    (header_path, cpp_path, makefile_path, demo_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: {} <rust_file> <module_name>",
            args[0]
        );
        std::process::exit(1);
    }

    let rust_file_path = &args[1];
    let module_name = &args[2];

    let (h_template_path, cpp_template_path, makefile_template_path, demo_template_path) = prepare_template_paths();

    let rust_source = fs::read_to_string(rust_file_path).expect("Failed to read Rust source file");

    let functions = parser::parse_exported_functions(&rust_source);

    let h_bindings = codegen::generate_header(&module_name, &functions, h_template_path.to_str().expect("Failed to get header template path str"));
    let cpp_bindings = codegen::generate_cpp(&module_name, &functions, cpp_template_path.to_str().expect("Failed to get source template path str"));
    let makefile = codegen::generate_makefile(&module_name, makefile_template_path.to_str().expect("Failed to get makefile template path str"));
    let demo = codegen::generate_demo(&module_name, demo_template_path.to_str().expect("Failed to get demp template path str"));

    let (h_path, cpp_path, mf_path, demo_path) = prepare_output_paths(module_name);

    fs::write(&h_path, h_bindings).expect(&format!("Failed to write {}", h_path.display()));
    fs::write(&cpp_path, cpp_bindings).expect(&format!("Failed to write {}", cpp_path.display()));
    fs::write(&mf_path, makefile).expect(&format!("Failed to write {}", mf_path.display()));
    fs::write(&demo_path, demo).expect(&format!("Failed to write {}", demo_path.display()));

    println!("Generated: {}, {}, {}, {}", h_path.display(), cpp_path.display(), mf_path.display(), demo_path.display());
}
