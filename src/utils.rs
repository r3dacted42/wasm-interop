use std::fs;
use std::path::PathBuf;

// For cpp → rust binder
pub fn prepare_cpp_template_paths() -> (PathBuf, PathBuf, PathBuf, PathBuf) {
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

pub fn prepare_cpp_output_paths(module_name: &str) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
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

// for rust to cpp binder
pub fn prepare_rust_template_paths() -> (PathBuf, PathBuf) {
    let templ_dir_path = PathBuf::from("./src/templates");
    if !templ_dir_path.exists() {
        panic!("Templates directory not found");
    }
    let rust_bindings_templ = templ_dir_path.join("rs_lib.hbs");
    let cargo_templ = templ_dir_path.join("rs_cargo.hbs");

    (rust_bindings_templ, cargo_templ)
}

pub fn prepare_rust_output_paths(module_name: &str) -> (PathBuf, PathBuf) {
    let dir_path = PathBuf::from(format!("{}-bindings", module_name));
    let src_path = dir_path.join("src");
    if !src_path.exists() {
        fs::create_dir_all(&src_path).expect("Failed to create output directory");
    }
    let bindings_path = src_path.join("lib.rs");
    let cargo_path = dir_path.join("Cargo.toml");

    (bindings_path, cargo_path)

}