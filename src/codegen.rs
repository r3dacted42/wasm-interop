use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;
use crate::parser::FunctionSignature;

/// Generate C++ bindings from Rust function signatures
pub fn generate_cpp_bindings(
    functions: Vec<FunctionSignature>,
    output_dir: &Path,
    module_name: &str,
) -> Result<()> {
    let mut handlebars = Handlebars::new();
    
    // Get the path to the templates directory in a platform-agnostic way
    let template_dir = Path::new("src").join("templates");
    
    // Register templates with platform-agnostic paths
    let header_template_path = template_dir.join("cpp_header.template");
    let impl_template_path = template_dir.join("cpp_impl.template");
    let makefile_template_path = template_dir.join("makefile.template");
    
    // Read template files
    let header_template = fs::read_to_string(&header_template_path)?;
    let impl_template = fs::read_to_string(&impl_template_path)?;
    let makefile_template = fs::read_to_string(&makefile_template_path)?;
    
    // Register templates
    handlebars.register_template_string("header", header_template)?;
    handlebars.register_template_string("impl", impl_template)?;
    handlebars.register_template_string("makefile", makefile_template)?;
    
    // Prepare data for templates
    let data = json!({
        "module_name": module_name,
        "functions": functions,
        "has_string_param": has_string_param(&functions),
        "header_guard": module_name.to_uppercase(),
    });
    
    // Generate header file
    let header_path = output_dir.join(format!("{}.hpp", module_name));
    let header_content = handlebars.render("header", &data)?;
    fs::write(header_path, header_content)?;
    
    // Generate implementation file
    let impl_path = output_dir.join(format!("{}.cpp", module_name));
    let impl_content = handlebars.render("impl", &data)?;
    fs::write(impl_path, impl_content)?;
    
    // Generate Makefile
    let makefile_path = output_dir.join("Makefile");
    let makefile_content = handlebars.render("makefile", &data)?;
    fs::write(makefile_path, makefile_content)?;
    
    // Generate a simple example
    generate_example(output_dir, module_name, &functions)?;
    
    Ok(())
}

/// Check if any function has string parameters
fn has_string_param(functions: &[FunctionSignature]) -> bool {
    functions.iter().any(|func| {
        func.args.iter().any(|arg| arg.cpp_type.contains("string")) ||
        func.cpp_return_type.contains("string")
    })
}

/// Generate a simple example using the bindings
fn generate_example(output_dir: &Path, module_name: &str, functions: &[FunctionSignature]) -> Result<()> {
    let examples_dir = output_dir.join("example");
    fs::create_dir_all(&examples_dir)?;
    
    // Generate example C++ file
    let example_path = examples_dir.join("example.cpp");
    
    // Create relative path to header file (platform agnostic)
    let header_path = PathBuf::from("..").join(format!("{}.hpp", module_name));
    let header_include = header_path.to_string_lossy().replace('\\', "/"); // For C++ #include
    
    let mut example_content = format!(r#"#include <iostream>
#include "{}"

int main() {{
    std::cout << "Loading WebAssembly module..." << std::endl;
    
    // Initialize the module
    {}Module module;
    
    if (!module.load()) {{
        std::cerr << "Failed to load module" << std::endl;
        return 1;
    }}
    
    std::cout << "Module loaded successfully" << std::endl;
    
"#, header_include, module_name);

    // Add example calls for each function
    for func in functions {
        example_content.push_str(&format!("    // Call {}(", func.name));
        
        let mut call_args = Vec::new();
        let mut example_args = Vec::new();
        
        for arg in &func.args {
            call_args.push(arg.name.clone());
            
            match arg.cpp_type.as_str() {
                "int8_t" | "int16_t" | "int32_t" | "int64_t" => example_args.push("42".to_string()),
                "uint8_t" | "uint16_t" | "uint32_t" | "uint64_t" => example_args.push("42".to_string()),
                "float" | "double" => example_args.push("3.14".to_string()),
                "bool" => example_args.push("true".to_string()),
                "std::string" | "const std::string&" => example_args.push(r#""Hello from C++""#.to_string()),
                _ => example_args.push(format!("/* provide a {} value here */", arg.cpp_type)),
            }
        }
        
        example_content.push_str(&example_args.join(", "));
        example_content.push_str(")\n");
        
        if func.cpp_return_type != "void" {
            example_content.push_str(&format!("    {} result{} = module.{}({});\n", 
                func.cpp_return_type, 
                if func.name == functions[0].name { "" } else { &func.name },
                func.name, 
                example_args.join(", ")));
                
            if func.cpp_return_type == "std::string" {
                example_content.push_str(&format!("    std::cout << \"{}() returned: \" << result{} << std::endl;\n\n", 
                    func.name,
                    if func.name == functions[0].name { "" } else { &func.name }));
            } else {
                example_content.push_str(&format!("    std::cout << \"{}() returned: \" << result{} << std::endl;\n\n", 
                    func.name,
                    if func.name == functions[0].name { "" } else { &func.name }));
            }
        } else {
            example_content.push_str(&format!("    module.{}({});\n", func.name, example_args.join(", ")));
            example_content.push_str(&format!("    std::cout << \"{}() called\" << std::endl;\n\n", func.name));
        }
    }
    
    example_content.push_str(r#"    return 0;
}
"#);

    fs::write(example_path, example_content)?;
    
    let example_makefile_path = examples_dir.join("Makefile");
    let cpp_impl_path = format!("../{}.cpp", module_name);
    
    let example_makefile_content = format!(r#"CXX = g++
CXXFLAGS = -std=c++17 -I..

example: example.cpp {}
	$(CXX) $(CXXFLAGS) -o example example.cpp {} -lwebassembly -lemscripten

clean:
	rm -f example
"#, cpp_impl_path, cpp_impl_path);

    fs::write(example_makefile_path, example_makefile_content)?;
    
    Ok(())
}