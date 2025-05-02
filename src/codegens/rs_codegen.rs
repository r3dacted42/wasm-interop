use crate::cpp_parser::CppFunction;
use std::fs;
use handlebars::Handlebars;

pub fn generate_bindings(module: &str, functions: &[CppFunction], template_path: &str) -> String {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("rs_lib", template_path)
        .expect("Failed to register Rust bindings template");

    let context = serde_json::json!({
        "module": module,
        "functions": functions.iter().map(|f| {
            serde_json::json!({
                "name": f.name,
                "return_type": map_cpp_type_to_rust(&f.return_type),
                "args": f.parameters.iter().map(|(typ, name)| {
                    serde_json::json!({
                        "type": map_cpp_type_to_rust(typ),
                        "name": name
                    })
                }).collect::<Vec<_>>()
            })
        }).collect::<Vec<_>>()
    });

    handlebars
        .render("rs_lib", &context)
        .expect("Failed to render Rust bindings")
}

pub fn generate_cargo(module: &str, template_path: &str) -> String {
    let template = fs::read_to_string(template_path)
        .expect("Failed to read Cargo.toml template");

    // Replace module in the template using the correct placeholders
    template.replace("{{module}}", module)
}

// Basic C++ to Rust type mapping
fn map_cpp_type_to_rust(cpp_type: &str) -> &str {
    match cpp_type.trim() {
        "int" => "i32",
        "float" => "f32",
        "double" => "f64",
        "void" => "()",
        "char*" | "const char*" => "*const u8", // rough approximation
        "bool" => "bool",
        _ => "i32", // fallback
    }
}
