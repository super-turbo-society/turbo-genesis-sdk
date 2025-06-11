use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn game(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_ident = input.ident.clone();

    let expanded = quote! {
        #input

        #[no_mangle]
        #[cfg(turbo_hot_reload)]
        pub unsafe extern "C" fn run() {
            use turbo::sys;

            let mut state = match sys::load() {
                Ok(bytes) => <#struct_ident>::try_from_slice(&bytes).unwrap_or_else(|_| #struct_ident::new()),
                Err(_) => #struct_ident::new(),
            };

            state.update();

            if let Ok(bytes) = state.try_to_vec() {
                let _ = sys::save(&bytes);
            }
        }
        
        #[no_mangle]
        #[cfg(not(turbo_hot_reload))]
        pub unsafe extern "C" fn run() {            
            static mut GAME_STATE: Option<#struct_ident> = None;

            let mut state = GAME_STATE.take().unwrap_or_else(#struct_ident::new);

            state.update();

            GAME_STATE = Some(state);
        }
    };

    TokenStream::from(expanded)
}
