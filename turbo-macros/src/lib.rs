use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Item, LitStr,
};

// =============================================================================
// Serialize
// =============================================================================

#[proc_macro_attribute]
pub fn serialize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    match &input {
        Item::Struct(_) => (),
        Item::Enum(_) => (),
        _ => {
            return quote! {
                compile_error!("#[turbo::serialize] only supports structs and enums.");
            }
            .into();
        }
    };

    let expanded = quote! {
        #[derive(
            Debug,
            Clone,
            turbo::borsh::BorshDeserialize,
            turbo::borsh::BorshSerialize,
            turbo::serde::Deserialize,
            turbo::serde::Serialize,
        )]
        #[borsh(crate = "turbo::borsh")]
        #[serde(crate = "turbo::serde")]
        #input
    };

    TokenStream::from(expanded)
}

// =============================================================================
// Game
// =============================================================================

#[proc_macro_attribute]
pub fn game(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    // clone the ident early to avoid borrowing issues
    let (ident, has_no_fields, is_unit_struct) = match &input {
        Item::Struct(s) => (
            s.ident.clone(),
            s.fields.is_empty(),
            matches!(s.fields, syn::Fields::Unit),
        ),
        Item::Enum(e) => (e.ident.clone(), false, false),
        _ => {
            return quote! {
                compile_error!("#[turbo::game] only supports structs and enums.");
            }
            .into();
        }
    };

    let init = if has_no_fields {
        if is_unit_struct {
            quote! { #ident }
        } else {
            quote! { #ident {} }
        }
    } else {
        quote! { #ident::new() }
    };

    let expanded = quote! {
        #[turbo::serialize]
        #input

        #[no_mangle]
        #[cfg(turbo_hot_reload)]
        pub unsafe extern "C" fn run() {
            use turbo::borsh::*;

            let mut state = match hot::load() {
                Ok(bytes) => <#ident>::try_from_slice(&bytes).unwrap_or_else(|_| #init),
                Err(_) => #init,
            };

            state.update();

            if let Ok(bytes) = borsh::to_vec(&state) {
                let _ = hot::save(&bytes);
            }
        }

        #[no_mangle]
        #[cfg(not(turbo_hot_reload))]
        pub unsafe extern "C" fn run() {
            static mut GAME_STATE: Option<#ident> = None;

            let mut state = GAME_STATE.take().unwrap_or_else(|| #init);

            state.update();

            GAME_STATE = Some(state);
        }
    };

    TokenStream::from(expanded)
}

// =============================================================================
// Channel
// =============================================================================

#[proc_macro_attribute]
pub fn channel(attr: TokenStream, item: TokenStream) -> TokenStream {
    channel::channel(attr, item)
}

mod channel {
    use super::*;
    struct ChannelArgs {
        name: String,
    }
    impl Parse for ChannelArgs {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let mut name = None;

            while !input.is_empty() {
                let ident: syn::Ident = input.parse()?;
                let _: syn::Token![=] = input.parse()?;
                let value: LitStr = input.parse()?;

                match ident.to_string().as_str() {
                    "name" => name = Some(value.value()),
                    _ => return Err(syn::Error::new_spanned(ident, "unexpected attribute key")),
                }

                if input.peek(syn::Token![,]) {
                    let _: syn::Token![,] = input.parse()?;
                }
            }

            let name = name.ok_or_else(|| input.error("missing `name`"))?;

            Ok(Self { name })
        }
    }
    /// Macro attribute to generate the extern handler
    pub fn channel(attr: TokenStream, item: TokenStream) -> TokenStream {
        let args = parse_macro_input!(attr as ChannelArgs);
        let input = parse_macro_input!(item as Item);

        let export_name = format!("channel/{}", args.name);
        let ident = match &input {
            Item::Struct(s) => &s.ident,
            Item::Enum(e) => &e.ident,
            _ => return quote!(compile_error!("Must be used on a struct or enum");).into(),
        };

        let channel_name = format!("{}", args.name);
        let channel_link_section_ident = format_ident!("_channel_link_section_{}", channel_name);
        let channel_extern_fn_ident = format_ident!("_channel_extern_fn_{}", channel_name);

        // Generate byte arrays
        let channel_name_bytes = channel_name.as_bytes().iter().map(|b| quote! { #b });
        let channel_name_len = channel_name_bytes.len();

        let expanded = quote! {
            // Register channel under turbo_programs custom section
            // Entry format: <program>/channels/<channel>,
            // Note: The trailing comma is important
            const _: () = {
                const LEN: usize = PROGRAM_NAME_BYTES.len() + 10 + #channel_name_len + 1;
                const fn build_bytes() -> [u8; LEN] {
                    let mut tmp = [0u8; LEN];
                    let mut base = 0;
                    // Insert program name
                    let pname = PROGRAM_NAME_BYTES;
                    let mut i = 0;
                    while i < pname.len() {
                        tmp[base + i] = pname[i];
                        i += 1;
                    }
                    base += pname.len();
                    // Insert namespace
                    let namespace = b"/channels/";
                    let mut i = 0;
                    while i < namespace.len() {
                        tmp[base + i] = namespace[i];
                        i += 1;
                    }
                    base += namespace.len();
                    // Insert channel name
                    let chan = [#(#channel_name_bytes),*];
                    let mut i = 0;
                    while i < chan.len() {
                        tmp[base + i] = chan[i];
                        i += 1;
                    }
                    base += chan.len();
                    // Insert trailing comma
                    tmp[base] = b',';
                    tmp
                }

                #[doc(hidden)]
                #[used]
                #[link_section = "turbo_programs"]
                #[allow(non_upper_case_globals)]
                pub static #channel_link_section_ident: [u8; LEN] = build_bytes();
            };

            #[unsafe(export_name = #export_name)]
            unsafe extern "C" fn #channel_extern_fn_ident() {
                use turbo::os::server;
                use server::channel::*;
                let handler = &mut #ident::new();
                let settings = &mut ChannelSettings::default();
                handler.on_open(settings);
                let timeout = settings.interval.unwrap_or(u32::MAX).max(16); // cap timeout at 16ms
                loop {
                    match recv_with_timeout(timeout) {
                        Ok(ChannelMessage::Connect(user_id, _data)) => {
                            handler.on_connect(user_id);
                        }
                        Ok(ChannelMessage::Disconnect(user_id, _data)) => {
                            handler.on_disconnect(user_id);
                        }
                        Ok(ChannelMessage::Data(user_id, data)) => {
                            match #ident::parse(&data) {
                                Ok(data) => handler.on_data(user_id, data),
                                Err(err) => server::log!("Error parsing data from user {user_id}: {err:?}"),
                            }
                        }
                        Err(ChannelError::Timeout) => {
                            handler.on_interval();
                        }
                        Err(_) => {
                            handler.on_close();
                            return
                        },
                    }
                }
            }

            impl #ident {
                pub fn subscribe() -> Option<turbo::os::client::channel::ChannelConnection<
                    <Self as turbo::os::server::channel::ChannelHandler>::Recv,
                    <Self as turbo::os::server::channel::ChannelHandler>::Send,
                >> {
                    turbo::os::client::channel::Channel::<
                        <Self as turbo::os::server::channel::ChannelHandler>::Recv,
                        <Self as turbo::os::server::channel::ChannelHandler>::Send,
                    >::subscribe(PROGRAM_ID, #channel_name, "*")
                }
            }

            #[turbo::serialize]
            #input
        };

        TokenStream::from(expanded)
    }
}

// =============================================================================
// Command
// =============================================================================

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    command::command(attr, item)
}

mod command {
    use super::*;
    struct CommandArgs {
        name: String,
    }
    impl Parse for CommandArgs {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let mut name = None;

            while !input.is_empty() {
                let ident: syn::Ident = input.parse()?;
                let _: syn::Token![=] = input.parse()?;
                let value: LitStr = input.parse()?;

                match ident.to_string().as_str() {
                    "name" => name = Some(value.value()),
                    _ => return Err(syn::Error::new_spanned(ident, "unexpected attribute key")),
                }

                if input.peek(syn::Token![,]) {
                    let _: syn::Token![,] = input.parse()?;
                }
            }

            let name = name.ok_or_else(|| input.error("missing `name`"))?;

            Ok(Self { name })
        }
    }

    pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
        let args = parse_macro_input!(attr as CommandArgs);
        let input = parse_macro_input!(item as Item);

        let export_name = format!("turbo/{}", args.name);
        let ident = match &input {
            Item::Struct(s) => &s.ident,
            Item::Enum(e) => &e.ident,
            _ => return quote!(compile_error!("Must be used on a struct or enum");).into(),
        };

        //
        let command_name = format!("{}", args.name);
        let command_link_section_ident = format_ident!("_command_link_section_{}", command_name);
        let command_extern_fn_ident = format_ident!("_command_extern_fn_{}", command_name);

        // Generate byte arrays
        let command_name_bytes = command_name.as_bytes().iter().map(|b| quote! { #b });
        let command_name_len = command_name_bytes.len();

        let expanded = quote! {
            // Register command under turbo_programs custom section
            // Entry format: <program>/commands/<command>,
            // Note: The trailing comma is important
            const _: () = {
                const LEN: usize = PROGRAM_NAME_BYTES.len() + 10 + #command_name_len + 1;
                const fn build_bytes() -> [u8; LEN] {
                    let mut tmp = [0u8; LEN];
                    let mut base = 0;
                    // Insert program name
                    let pname = PROGRAM_NAME_BYTES;
                    let mut i = 0;
                    while i < pname.len() {
                        tmp[base + i] = pname[i];
                        i += 1;
                    }
                    base += pname.len();
                    // Insert namespace
                    let namespace = b"/commands/";
                    let mut i = 0;
                    while i < namespace.len() {
                        tmp[base + i] = namespace[i];
                        i += 1;
                    }
                    base += namespace.len();
                    // Insert command name
                    let chan = [#(#command_name_bytes),*];
                    let mut i = 0;
                    while i < chan.len() {
                        tmp[base + i] = chan[i];
                        i += 1;
                    }
                    base += chan.len();
                    // Insert trailing comma
                    tmp[base] = b',';
                    tmp
                }

                #[doc(hidden)]
                #[used]
                #[link_section = "turbo_programs"]
                #[allow(non_upper_case_globals)]
                pub static #command_link_section_ident: [u8; LEN] = build_bytes();
            };

            #[turbo::serialize]
            #input

            impl #ident {
                pub fn exec(self) -> String {
                    turbo::os::client::command::exec(PROGRAM_ID, #command_name, self)
                }
            }

            #[cfg(turbo_no_run)]
            #[unsafe(export_name = #export_name)]
            unsafe extern "C" fn #command_extern_fn_ident() -> usize {
                let user_id = turbo::os::server::command::user_id();
                let mut cmd = turbo::os::server::command::parse_input::<#ident>();
                match &mut cmd {
                    Ok(cmd) => match cmd.run(&user_id) {
                        Ok(_) => turbo::os::server::command::COMMIT,
                        Err(err) => {
                            turbo::os::server::log!("Command failed: {:?}", err);
                            turbo::os::server::command::CANCEL
                        }
                    },
                    Err(err) => {
                        turbo::os::server::log!("Failed to parse command bytes: {:?}", err);
                        turbo::os::server::command::CANCEL
                    },
                }
            }

        };

        TokenStream::from(expanded)
    }
}

#[proc_macro_attribute]
pub fn program(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD as b64_url_safe, Engine};
    use sha2::{Digest, Sha256};

    // Load metadata
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_toml = std::fs::read_to_string(format!("{manifest}/Cargo.toml")).unwrap();
    let parsed: toml_edit::DocumentMut = cargo_toml.parse().unwrap();
    let user_id = parsed["package"]["metadata"]["turbo"]["user"]
        .as_str()
        .expect("Missing [package.metadata.turbo] user entry");

    // Parse module
    let mut module = syn::parse_macro_input!(item as syn::ItemMod);
    let ident = &module.ident;
    let program_name = ident.to_string();

    // Hash for program ID
    let mut hasher = Sha256::new();
    let uuid = uuid::Uuid::parse_str(user_id)
        .expect("Invalid UUID format in [package.metadata.turbo.user]");
    hasher.update(uuid.as_bytes());
    hasher.update(program_name.as_bytes());
    let program_id = b64_url_safe.encode(hasher.finalize());

    // Generate byte arrays
    let name_bytes = program_name.as_bytes().iter().map(|b| quote! { #b });

    // Inject into the module body
    let inject = quote! {
        pub const PROGRAM_NAME_BYTES: [u8; #program_name.len()] = [#(#name_bytes),*];
        pub const PROGRAM_NAME: &str = #program_name;
        pub const PROGRAM_OWNER: &str = #user_id;
        pub const PROGRAM_ID: &str = #program_id;

        pub fn watch<T: turbo::borsh::BorshDeserialize>(key: impl AsRef<std::path::Path>) -> Option<T> {
            let path = std::path::Path::new(PROGRAM_ID).join(key.as_ref());
            turbo::os::client::fs::watch(path).parse()
        }
    };

    match &mut module.content {
        Some((_, items)) => {
            let parsed_items: Vec<syn::Item> = syn::parse2::<syn::File>(inject)
                .expect("failed to parse inject block")
                .items;

            items.splice(0..0, parsed_items);
        }
        None => {
            return quote!(compile_error!("Module must have a body")).into();
        }
    }

    quote! {
        #module
    }
    .into()
}
