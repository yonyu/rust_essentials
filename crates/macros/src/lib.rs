use proc_macro;
//extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::ItemFn;

use syn::{parse_macro_input, DeriveInput};

// a simple declarative macro
macro_rules! say_hello {
    () => {
        println!("Hello from macros!");
    }
}

// a simple function-like procedural macro
#[proc_macro]
pub fn generate_function(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as ItemFn);

    // Extract the function name, attributes, and block
    let func_name = &input.sig.ident;
    let func_attrs = &input.attrs;
    let func_block = &input.block;

    // Generate the output tokens
    let expanded = quote! {
        #(#func_attrs)*
        fn #func_name() {
            println!("{}", stringify!(#func_name));
            #func_block
        }
    };

    // Convert the generated tokens into a TokenStream
    TokenStream::from(expanded)
}


// #[proc_macro]
// pub fn generate_function(input: TokenStream) -> TokenStream {
//     // Parse the input tokens into a syntax tree
//     let input = parse_macro_input!(input as ItemFn);
//
//     // Extract the function name and block
//     let func_name = &input.sig.ident;
//     let func_block = &input.block;
//
//     // Generate the output tokens
//     let expanded = quote! {
//         fn #func_name() {
//             println!("{}", stringify!(#func_name));
//         }
//
//         #func_block
//     };
//
//     // Convert the generated tokens into a TokenStream
//     TokenStream::from(expanded)
// }

// #[proc_macro]
// pub fn generate_function(input: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(input as syn::Ident);
//     let func_name = input.to_string();
//     let output = quote! {
//         fn #func_name() {
//             println!("{}", #input);
//         }
//     };
//     output.into()
// }

// #[proc_macro]
// pub fn generate_function(input: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(input as syn::Ident);
//     let func_name = input.to_string();
//     let output = quote! {
//         fn #func_name() {
//             println!("{}", #input);
//         }
//     };
//
//     output.into()
// }



// an attribute macro that adds a hello function to a struct
#[proc_macro_attribute]
pub fn hello_macro(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl #name {
            pub fn hello() {
                println!("Hello from {}", stringify!(#name));
            }
        }
        #input
    };
    expanded.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello() {
        say_hello!();
    }
}
