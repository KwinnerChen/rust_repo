extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 构建rust代码所代表的语法树
    // 一边可以进行操作
    let ast = syn::parse(input).unwrap();

    // 构建trait实现
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("hello, macro! my name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}