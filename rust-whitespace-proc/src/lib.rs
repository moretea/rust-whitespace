extern crate proc_macro;
extern crate rust_whitespace;
use proc_macro::{TokenStream};

#[proc_macro_attribute]
pub fn whitespace(_attr: TokenStream, item: TokenStream) -> TokenStream {
        let mut iter = item.into_iter();
        let tok_fn = iter.next().expect("No 'fn' token");
        let tok_fn_name = iter.next().expect("No function name token");
        let tok_fn_args = iter.next().expect("No function args token");
        let tok_fn_body= iter.next().expect("No function body token");

        let fn_body_text = tok_fn_body.span().source_text().expect("could not get function body");
        let _program = rust_whitespace::parse(&fn_body_text);
        
        let compiled_whitespace_code = r#"
            println!("Hello world!");
        "#.to_string();

        let parts = [tok_fn.to_string(), tok_fn_name.to_string(), tok_fn_args.to_string(), compiled_whitespace_code];
        let code = parts.join(" ");
        code.parse().expect("Expected to generate valid code!")
}