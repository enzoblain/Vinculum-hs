extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ExprLit, ItemFn, Lit, LitStr, MetaNameValue, parse, parse_macro_input};

#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let haskell_file = if !args.is_empty() {
        if let Ok(meta) = parse::<MetaNameValue>(args.clone()) {
            if meta.path.is_ident("haskell_file") {
                if let syn::Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) = &meta.value
                {
                    Some(lit_str.value())
                } else {
                    None
                }
            } else {
                None
            }
        } else if let Ok(lit) = parse::<LitStr>(args) {
            Some(lit.value())
        } else {
            None
        }
    } else {
        None
    };

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let expanded = if let Some(file_path) = haskell_file {
        quote! {
            #(#attrs)*
            #vis #sig {
                const WORKSPACE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
                let full_path = {
                    let root = std::path::Path::new(WORKSPACE_ROOT);
                    let workspace = root.parent()
                        .and_then(|p| p.parent())
                        .unwrap_or(root);
                    workspace.join(#file_path)
                        .canonicalize()
                        .ok()
                        .and_then(|p| p.to_str().map(|s| s.to_string()))
                        .unwrap_or_else(|| workspace.join(#file_path).to_string_lossy().to_string())
                };
                unsafe { std::env::set_var("HASKELL_FILE", full_path); }
                vinculum::runtime::init();
                #block
                vinculum::runtime::shutdown();
            }
        }
    } else {
        quote! {
            #(#attrs)*
            #vis #sig {
                vinculum::runtime::init();
                #block
                vinculum::runtime::shutdown();
            }
        }
    };

    TokenStream::from(expanded)
}
