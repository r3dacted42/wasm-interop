use anyhow::{Result, anyhow};
use std::path::Path;
use std::fs;
use syn::{Item, ItemFn, Attribute};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FunctionArg {
    pub name: String,
    pub rust_type: String,
    pub cpp_type: String,
}

#[derive(Debug, Serialize)]
pub struct FunctionSignature {
    pub name: String,
    pub args: Vec<FunctionArg>,
    pub return_type: String,
    pub cpp_return_type: String,
}

/// Parse a Rust file and extract all exported functions
pub fn parse_rust_file(path: &Path) -> Result<Vec<FunctionSignature>> {
    let content = fs::read_to_string(path)?;
    parse_rust_code(&content)
}

/// Parse Rust code string and extract all exported functions
pub fn parse_rust_code(code: &str) -> Result<Vec<FunctionSignature>> {
    let file = syn::parse_file(code)?;
    let mut functions = Vec::new();

    for item in file.items {
        if let Item::Fn(func) = item {
            if has_wasm_bindgen_attribute(&func.attrs) {
                match extract_function_signature(func) {
                    Ok(sig) => functions.push(sig),
                    Err(e) => println!("Warning: Skipping function - {}", e),
                }
            }
        }
    }

    Ok(functions)
}

/// Check if a function has the #[wasm_bindgen] attribute
fn has_wasm_bindgen_attribute(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().segments.iter().any(|segment| {
            segment.ident == "wasm_bindgen"
        })
    })
}

/// Extract function signature from ItemFn
fn extract_function_signature(func: ItemFn) -> Result<FunctionSignature> {
    let name = func.sig.ident.to_string();
    
    let mut args = Vec::new();
    for input in func.sig.inputs {
        if let syn::FnArg::Typed(pat_type) = input {
            let arg_name = match &*pat_type.pat {
                syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                _ => return Err(anyhow!("Unsupported parameter pattern")),
            };
            
            let rust_type = get_type_as_string(&pat_type.ty)?;
            let cpp_type = rust_to_cpp_type(&rust_type)?;
            
            args.push(FunctionArg {
                name: arg_name,
                rust_type,
                cpp_type,
            });
        }
    }
    
    let return_type = if let syn::ReturnType::Type(_, ty) = &func.sig.output {
        get_type_as_string(ty)?
    } else {
        "()".to_string()
    };
    
    let cpp_return_type = rust_to_cpp_type(&return_type)?;
    
    Ok(FunctionSignature {
        name,
        args,
        return_type,
        cpp_return_type,
    })
}

/// Convert Rust type to C++ type
fn rust_to_cpp_type(rust_type: &str) -> Result<String> {
    match rust_type {
        "i8" => Ok("int8_t".to_string()),
        "i16" => Ok("int16_t".to_string()),
        "i32" => Ok("int32_t".to_string()),
        "i64" => Ok("int64_t".to_string()),
        "u8" => Ok("uint8_t".to_string()),
        "u16" => Ok("uint16_t".to_string()),
        "u32" => Ok("uint32_t".to_string()),
        "u64" => Ok("uint64_t".to_string()),
        "f32" => Ok("float".to_string()),
        "f64" => Ok("double".to_string()),
        "bool" => Ok("bool".to_string()),
        "String" => Ok("std::string".to_string()),
        "&str" => Ok("const std::string&".to_string()),
        "()" => Ok("void".to_string()),
        _ if rust_type.starts_with("Option<") => {
            // Handle Option types (simplified)
            let inner = rust_type.trim_start_matches("Option<").trim_end_matches(">");
            let cpp_inner = rust_to_cpp_type(inner)?;
            Ok(format!("std::optional<{}>", cpp_inner))
        }
        _ => Err(anyhow!("Unsupported type: {}", rust_type)),
    }
}

/// Get string representation of a Rust type
fn get_type_as_string(ty: &syn::Type) -> Result<String> {
    match ty {
        syn::Type::Path(type_path) if !type_path.path.segments.is_empty() => {
            let segment = &type_path.path.segments[0];
            Ok(segment.ident.to_string())
        }
        syn::Type::Reference(type_ref) => {
            if let syn::Type::Path(type_path) = &*type_ref.elem {
                if !type_path.path.segments.is_empty() {
                    let segment = &type_path.path.segments[0];
                    return Ok(format!("&{}", segment.ident));
                }
            }
            Err(anyhow!("Unsupported reference type"))
        }
        _ => Err(anyhow!("Unsupported type")),
    }
}