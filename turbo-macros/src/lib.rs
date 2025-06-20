use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Item, ItemStruct};

#[proc_macro_attribute]
pub fn turbo_game(_attr: TokenStream, item: TokenStream) -> TokenStream {
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

#[proc_macro_attribute]
pub fn turbo_serialize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as Item);

    // clone the ident early to avoid borrowing issues
    let ident = match &input {
        Item::Struct(s) => s.ident.clone(),
        Item::Enum(e) => e.ident.clone(),
        _ => {
            return quote! {
                compile_error!("#[turbo_serialize] only supports structs and enums.");
            }
            .into();
        }
    };

    // mutable access to attrs without borrowing `input`
    let attrs = match &mut input {
        Item::Struct(s) => &mut s.attrs,
        Item::Enum(e) => &mut e.attrs,
        _ => unreachable!(), // already checked above
    };

    let extra_derives = quote! {
        ::borsh::BorshSerialize,
        ::borsh::BorshDeserialize,
        ::serde::Serialize,
        ::serde::Deserialize
    };

    let mut found_derive = false;

    for attr in attrs.iter_mut() {
        if attr.path().is_ident("derive") {
            found_derive = true;
            let new_attr: syn::Attribute = syn::parse_quote! {
                #[derive(::borsh::BorshSerialize, ::borsh::BorshDeserialize, ::serde::Serialize, ::serde::Deserialize)]
            };
            *attr = new_attr;
            break;
        }
    }

    if !found_derive {
        let new_attr: Attribute = syn::parse_quote! {
            #[derive(#extra_derives)]
        };
        attrs.push(new_attr);
    }

    let expanded = quote! {
        #input

        impl ::turbo_serialization::TurboSerializable for #ident {
            fn to_vec(&self) -> Vec<u8> {
                ::borsh::to_vec(self)
            }

            fn try_from_slice(bytes: &[u8]) -> borsh::io::Result<Self> {
                ::borsh::BorshDeserialize::try_from_slice(bytes)
            }

            fn to_json_string(&self) -> String {
                ::serde_json::to_string(self)
            }

            fn try_from_json_str(s: &str) -> Result<Self, ::serde_json::Error> {
                ::serde_json::from_str(s)
            }
        }
    };

    TokenStream::from(expanded)
}
