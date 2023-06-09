use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(_: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree representing a function
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract the name of the input function
    let fn_name = input_fn.sig.ident.clone();

    let output = quote! {
        #[export_name = "kernel_run"]
        pub extern "C" fn kernel_run() {
            let mut runtime = rock_n_rollup::core::KernelRuntime::default();
            let mut app = rock_n_rollup::core::Application::new(&mut runtime);
            #fn_name(&mut app);
        }

        #input_fn
    };

    // Return the generated tokens
    output.into()
}
