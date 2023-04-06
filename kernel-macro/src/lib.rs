use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, TypeParamBound};

#[proc_macro_attribute]
pub fn main(_: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree representing a function
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract the name of the input function
    let fn_name = input_fn.sig.ident.clone();

    let mut is_app = false;
    for param in input_fn.sig.generics.params.iter() {
        if let syn::GenericParam::Type(ty) = param {
            let bounds = &ty.bounds;
            let bounds = bounds.first();
            if let Some(where_clause) = bounds {
                if let TypeParamBound::Trait(predicates) = where_clause {
                    is_app = predicates.path.is_ident("App");
                }
            }
        }
    }

    let output = match is_app {
        true => {
            quote! {
                #[export_name = "kernel_run"]
                pub extern "C" fn kernel_run() {
                    let mut runtime = rock_n_rollup::KernelRuntime::new();
                    let mut app = rock_n_rollup::Application::new(&mut runtime);

                    #fn_name(&mut app);
                }

                #input_fn
            }
        }
        false => {
            quote! {
                #[export_name = "kernel_run"]
                pub extern "C" fn kernel_run() {
                    let mut runtime = rock_n_rollup::KernelRuntime::new();
                    #fn_name(&mut runtime);
                }

                #input_fn
            }
        }
    };

    // Print the generated Rust code for debugging purposes
    let output_string = output.to_string();
    println!("{}", output_string);

    // Return the generated tokens
    output.into()
}
