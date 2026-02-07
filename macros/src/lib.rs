use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn include_lines(input: TokenStream) -> TokenStream {
    let filename = parse_macro_input!(input as LitStr);
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src") // adjust path as needed
        .join(filename.value());

    let contents = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));

    let lines: Vec<String> = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_string())
        .collect();

    let tokens = quote! {
        [#(#lines),*]
    };

    tokens.into()
}
