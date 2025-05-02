use regex::Regex;

#[derive(Debug)]
pub struct CppFunction {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<(String, String)>, // (type, name)
}

pub fn parse_exported_functions(source: &str) -> Vec<CppFunction> {
    let mut exported_functions = vec![];

    // Regex to find EMSCRIPTEN_KEEPALIVE functions with return type, name, and parameter list
    let re = Regex::new(
        r"(?m)EMSCRIPTEN_KEEPALIVE\s+([a-zA-Z_][a-zA-Z0-9_:<>]*)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)"
    ).unwrap();

    for caps in re.captures_iter(source) {
        println!("Matched function: {:?}", &caps[0]);

        let return_type = caps[1].trim().to_string();
        let name = caps[2].trim().to_string();
        let param_list = caps[3].trim();

        println!("Extracted function name: {}", name);
        println!("Extracted return type: {}", return_type);
        println!("Raw parameter list: {}", param_list);

        let mut parameters = vec![];
        if !param_list.is_empty() {
            for param in param_list.split(',') {
                let parts: Vec<&str> = param.trim().split_whitespace().collect();
                if parts.len() >= 2 {
                    let param_type = parts[..parts.len()-1].join(" ");
                    let param_name = parts[parts.len()-1].to_string();
                    println!("Extracted parameter: {} {}", param_type, param_name);
                    parameters.push((param_type, param_name));
                } else {
                    println!("Warning: Unparsable parameter: {}", param);
                }
            }
        }

        let func = CppFunction {
            name,
            return_type,
            parameters,
        };

        println!("Parsed function: {:?}", func);
        exported_functions.push(func);
    }

    println!("Parsed functions: {:?}", exported_functions);
    exported_functions
}
