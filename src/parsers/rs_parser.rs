use syn::visit::Visit;
use syn::{Attribute, FnArg, ItemFn, PatType, ReturnType, Type};

#[derive(Debug)]
pub struct ExportedFunction {
    pub name: String,
    pub args: Vec<(String, String)>, // (arg_name, arg_type)
    pub return_type: Option<String>,
}

struct ExportedFnVisitor {
    pub functions: Vec<ExportedFunction>,
}

impl<'ast> Visit<'ast> for ExportedFnVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        for attr in &node.attrs {
            if is_export(attr) {
                let name = node.sig.ident.to_string();
                let mut args = vec![];

                for input in &node.sig.inputs {
                    if let FnArg::Typed(PatType { pat, ty, .. }) = input {
                        if let syn::Pat::Ident(pat_ident) = &**pat {
                            args.push((pat_ident.ident.to_string(), type_to_string(&*ty)));
                        }
                    }
                }

                let return_type = match &node.sig.output {
                    ReturnType::Default => None,
                    ReturnType::Type(_, ty) => Some(type_to_string(&*ty)),
                };

                self.functions.push(ExportedFunction {
                    name,
                    args,
                    return_type,
                });
            }
        }
    }
}

fn is_export(attr: &Attribute) -> bool {
    attr.path().is_ident("unsafe")
}

fn type_to_string(ty: &Type) -> String {
    quote::quote!(#ty).to_string()
}

pub fn parse_exported_functions(source: &str) -> Vec<ExportedFunction> {
    let syntax = syn::parse_file(source).expect("Failed to parse Rust file");
    let mut visitor = ExportedFnVisitor {
        functions: Vec::new(),
    };
    visitor.visit_file(&syntax);
    visitor.functions
}
