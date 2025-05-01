use crate::parser::ExportedFunction;
use handlebars::Handlebars;
use serde::Serialize;
use std::fs;

fn map_rust_type_to_cpp(rust_type: &str) -> String {
    match rust_type {
        "i32" => "int".to_string(),
        "u32" => "unsigned int".to_string(),
        "f32" => "float".to_string(),
        "f64" => "double".to_string(),
        "bool" => "bool".to_string(),
        "String" | "& str" => "const char*".to_string(),
        _ => "void*".to_string(),
    }
}

fn map_rust_type_to_wasmer_val(rust_type: &str) -> String {
    match rust_type {
        "i32" | "u32" => "i32".to_string(),
        "f32" => "f32".to_string(),
        "f64" => "f64".to_string(),
        _ => "i32".to_string(),
    }
}

fn map_wasmer_unwrap(rust_type: &str) -> String {
    match rust_type {
        "i32" | "u32" => "i32".to_string(),
        "f32" => "f32".to_string(),
        "f64" => "f64".to_string(),
        _ => "i32".to_string(),
    }
}

#[derive(Serialize)]
struct Arg {
    name: String,
    #[serde(rename = "type")]
    typ: String,
    idx: usize,
    wasm_type: String,
}

#[derive(Serialize)]
struct CppFunction {
    name: String,
    args: Vec<Arg>,
    arg_count: usize,
    return_type: String,
    wasm_unwrap: String,
}

#[derive(Serialize)]
struct TemplateData {
    module_name: String,
    header_name: String,
    functions: Vec<CppFunction>,
}

fn prepare_template_data(module_name: &str, functions: &[ExportedFunction]) -> TemplateData {
    let cpp_functions = functions
        .iter()
        .map(|f| {
            let mut idx = 0;
            let mut args = Vec::new();
            for (name, rust_type) in &f.args {
                let cpp_type = map_rust_type_to_cpp(rust_type);
                let wasm_type = map_rust_type_to_wasmer_val(rust_type);
                args.push(Arg {
                    name: name.clone(),
                    typ: cpp_type.clone(),
                    idx,
                    wasm_type,
                });
                idx += if cpp_type == "const char*" { 2 } else { 1 };
            }

            let return_type = f
                .return_type
                .as_ref()
                .map(|r| map_rust_type_to_cpp(r))
                .unwrap_or_else(|| "void".to_string());

            CppFunction {
                name: f.name.clone(),
                args,
                arg_count: idx,
                return_type,
                wasm_unwrap: f
                    .return_type
                    .as_ref()
                    .map(|r| map_wasmer_unwrap(r))
                    .unwrap_or_else(|| "i32".to_string()),
            }
        })
        .collect();

    TemplateData {
        module_name: module_name.to_string(),
        header_name: format!("{}.hpp", module_name),
        functions: cpp_functions,
    }
}

pub fn generate_cpp(module_name: &str, functions: &[ExportedFunction], cpp_template_path: &str) -> String {
    let data = prepare_template_data(module_name, functions);
    let mut handlebars = Handlebars::new();
    let cpp_template = fs::read_to_string(cpp_template_path).expect("Failed to read .cpp template");
    handlebars
        .register_template_string("cpp", cpp_template)
        .unwrap();
    handlebars
        .render("cpp", &data)
        .expect("Failed to render .cpp")
}

pub fn generate_header(module_name: &str, functions: &[ExportedFunction], h_template_path: &str) -> String {
    let data = prepare_template_data(module_name, functions);
    let mut handlebars = Handlebars::new();
    let h_template = fs::read_to_string(h_template_path).expect("Failed to read .h template");
    handlebars
        .register_template_string("header", h_template)
        .unwrap();
    handlebars
        .render("header", &data)
        .expect("Failed to render .h")
}

#[derive(Serialize)]
struct ModuleNameData {
    module_name: String,
}

pub fn generate_makefile(
    module_name: &str,
    template_path: &str,
) -> String {
    let data = ModuleNameData {
        module_name: module_name.to_string(),
    };

    let template_content =
        fs::read_to_string(template_path).expect("Failed to read Makefile template");

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("makefile", template_content)
        .unwrap();

    handlebars
        .render("makefile", &data)
        .expect("Failed to render Makefile")
}

pub fn generate_demo(
    module_name: &str,
    template_path: &str,
) -> String {
    let data = ModuleNameData {
        module_name: module_name.to_string(),
    };

    let template_content =
        fs::read_to_string(template_path).expect("Failed to read demo template");

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("demo", template_content)
        .unwrap();

    handlebars
        .render("demo", &data)
        .expect("Failed to render demo")
}