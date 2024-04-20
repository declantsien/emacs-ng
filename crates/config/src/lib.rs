#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
use std::error::Error;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn run() -> Result<(), Box<dyn Error>> {
    use std::any::type_name;
    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    let mut file = File::open(concat!(env!("OUT_DIR"), "/bindings.rs"))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let ast = syn::parse_file(&content)?;
    ast.items.into_iter().for_each(|item| {
        match item {
            syn::Item::Const(i) => match *i.expr {
                syn::Expr::Lit(lit) => {
                    let key = i.ident.to_string();
                    match lit.lit {
                        syn::Lit::ByteStr(s) => {
                            println!("cargo::rustc-env={}=\"{}\"", key.to_uppercase(), unsafe {
                                CString::from_vec_with_nul_unchecked(s.value())
                                    .into_string()
                                    .unwrap()
                            });
                        }
                        syn::Lit::Int(i) => {
                            let value = i.base10_parse::<i32>().map_or(0, |i| i);
                            if value > 0 {
                                println!("cargo::rustc-cfg={}", key.to_lowercase());
                            }
                        }
                        _ => {}
                    };
                }
                _ => {}
            },
            _ => {}
        };
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = run();
        assert_eq!(result.is_ok(), true);
    }
}
