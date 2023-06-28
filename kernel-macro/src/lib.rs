use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

// procedural macros allow to define custom attribute-like syntax
// and perform code transformations at compile-time
#[proc_macro_attribute]
// this is a function signature of procedural macro attribute
// it takes 2 paramters, representing the attribute arguments
// and annotated function's code and return a TokenStream
pub fn main(_: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree representing a function.
    // Or take the input token stream and performs the neccessary parsing
    // and validation to convert it into a Rust syntax tree.
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extract the name of the input function by accessing its signatures
    // sig and then retrieving the identifier 'ident'
    // the clone method is used to create a clone of the function name
    let fn_name = input_fn.sig.ident.clone();

    // quote macro to generate the tranformed code.
    // it constructs a syntax tree using the provided tokens inside the quote block.
    // the generated code includes the transformed function definition and
    // additional function named 'kernel_run'
    let output = quote! {
        #[export_name = "kernel_run"]
        pub extern "C" fn kernel_run() {
            /// Maybe you can see the implementation of
            /// https://gitlab.com/tezos/tezos/-/blob/master/src/kernel_sdk/entrypoint/src/lib.rs
            /// Create a new RollupHost
            /// And give it to the KernelRuntime::new(rollup_host)
            //let mut runtime = rock_n_rollup::core::KernelRuntime::default();
            //use $crate::tezos_smart_rollup_core::RollupHost;
            let mut host = unsafe{tezos_smart_rollup_core::rollup_host::RollupHost::new()};
            let mut runtime = rock_n_rollup::core::KernelRuntime::new(host);
            let mut app = rock_n_rollup::core::Application::new(&mut runtime);

            // invokes the original input function (fn_name) with a mutable reference to the app variable
            #fn_name(&mut app);
        }

        // this includes the original input function in the generated code
        #input_fn
    };

    // Return the generated tokens
    // convert the output syntax tree into a TokenStream and return it
    // as the result of the procedural macro attribute
    output.into()
}
