use std::path::{Path, PathBuf};

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    Item, ItemEnum, ItemMod, ItemStruct, LitStr,
};
use turbo_genesis_abi::{
    TurboProgramChannelMetadata, TurboProgramCommandMetadata, TurboProgramMetadata,
};

// =============================================================================
// Serialize
// =============================================================================

#[proc_macro_attribute]
pub fn serialize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    match &input {
        Item::Struct(_) | Item::Enum(_) => (),
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
        #[cfg(all(turbo_hot_reload, not(turbo_no_run)))]
        pub unsafe extern "C" fn run() {
            use turbo::borsh::*;

            let mut state = match hot::load() {
                Ok(bytes) => <#ident>::try_from_slice(&bytes).unwrap_or_else(|_| #init),
                Err(err) => {
                    log!("[turbo] Hot reload deserialization failed: {err:?}");
                    #init
                },
            };

            state.update();

            if let Ok(bytes) = borsh::to_vec(&state) {
                let Err(err) = hot::save(&bytes) = {
                    log!("[turbo] hot save failed: Error code {err}");
                }
            }
        }

        #[no_mangle]
        #[cfg(all(not(turbo_hot_reload), not(turbo_no_run)))]
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
// Command
// =============================================================================

#[proc_macro_attribute]
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item // Marker only
}

#[derive(Debug)]
struct CommandArgs {
    pub name: String,
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

// =============================================================================
// Channel
// =============================================================================

#[proc_macro_attribute]
pub fn channel(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item // Marker only
}

#[derive(Debug)]
struct ChannelArgs {
    pub name: String,
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

// =============================================================================
// Program
// =============================================================================

#[proc_macro_attribute]
pub fn program(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD as b64_url_safe, Engine};
    use sha2::{Digest, Sha256};

    // Parse the module this macro is attached to
    let mut module = parse_macro_input!(item as ItemMod);
    let program_name = module.ident.to_string();

    // --------------------------------------------------------------------------
    // Load and parse the user's UUID from Cargo.toml (under metadata.turbo.user)
    // This acts as a unique owner identity for namespacing the program ID
    // --------------------------------------------------------------------------
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_dir = Path::new(&manifest_dir);
    let cargo_toml_path = project_dir.join("Cargo.toml");
    let cargo_toml = std::fs::read_to_string(&cargo_toml_path).unwrap();
    let parsed: toml_edit::DocumentMut = cargo_toml.parse().unwrap();
    let user_id = parsed["package"]["metadata"]["turbo"]["user"]
        .as_str()
        .expect("Missing [package.metadata.turbo] user entry");

    // Hash UUID and program name together to create a stable, unique PROGRAM_ID
    let uuid = uuid::Uuid::parse_str(user_id).expect("Invalid UUID format");
    let mut hasher = Sha256::new();
    hasher.update(uuid.as_bytes());
    hasher.update(program_name.as_bytes());
    let program_id = b64_url_safe.encode(hasher.finalize());

    // -----------------------------------------------------------------------
    // Process the module content if it is inline (not out-of-line `mod xyz;`)
    // -----------------------------------------------------------------------
    if let Some((_, items)) = &mut module.content {
        // Initialize program metadata
        let mut program_metadata = TurboProgramMetadata {
            name: program_name.clone(),
            program_id: program_id.clone(),
            owner_id: user_id.to_string(),
            commands: vec![],
            channels: vec![],
        };

        // Iterate over each item in the module
        let new_items = match process_program_module_items(&mut program_metadata, items) {
            Ok(items) => items,
            Err(err) => return err.to_compile_error().into(),
        };

        // let mut new_items = vec![];
        // for item in items.drain(..) {
        //     match &item {
        //         Item::Mod(m) => {
        //             // Only care about `#[program_submodule]`
        //             let is_submodule = m
        //                 .attrs
        //                 .iter()
        //                 .any(|attr| attr.path().is_ident("program_submodule"));

        //             if let Some(iitems) = m.content {}
        //         }
        //         Item::Struct(ItemStruct { ident, attrs, .. })
        //         | Item::Enum(ItemEnum { ident, attrs, .. }) => {
        //             let mut handled = false;

        //             for attr in attrs {
        //                 // --------------------------------------------------------------------
        //                 // #[command(name = "...")] — generate FFI + exec client binding
        //                 // --------------------------------------------------------------------
        //                 const COMMAND_ATTR_IDENT: &str = "command";
        //                 if attr
        //                     .path()
        //                     .segments
        //                     .last()
        //                     .map_or(false, |seg| seg.ident == COMMAND_ATTR_IDENT)
        //                 {
        //                     // Parse command name
        //                     let args = match attr.parse_args::<CommandArgs>() {
        //                         Ok(args) => args,
        //                         Err(err) => return err.to_compile_error().into(),
        //                     };
        //                     let items =
        //                         process_program_command(&mut program_metadata, args, &item, ident);
        //                     new_items.extend(items);
        //                     handled = true;
        //                     break;
        //                 }

        //                 // --------------------------------------------------------------------
        //                 // #[channel(name = "...")] — generate FFI + subscribe method
        //                 // --------------------------------------------------------------------
        //                 const CHANNEL_ATTR_IDENT: &str = "channel";
        //                 if attr
        //                     .path()
        //                     .segments
        //                     .last()
        //                     .map_or(false, |seg| seg.ident == CHANNEL_ATTR_IDENT)
        //                 {
        //                     // Parse channel name
        //                     let args = match attr.parse_args::<ChannelArgs>() {
        //                         Ok(args) => args,
        //                         Err(err) => return err.to_compile_error().into(),
        //                     };
        //                     let items =
        //                         process_program_channel(&mut program_metadata, args, &item, ident);
        //                     new_items.extend(items);
        //                     handled = true;
        //                     break;
        //                 }
        //             }

        //             // If not a #[command] or #[channel], emit as-is
        //             if !handled {
        //                 new_items.push(item);
        //             }
        //         }
        //         // Non-struct/enum items (e.g. impls, consts) are passed through unchanged
        //         _ => new_items.push(item),
        //     }
        // }

        // Replace module body with rewritten contents
        *items = new_items;

        // ----------------------------------------------------------------
        // Inject program-related metadata, constants, modules, and helpers
        // ----------------------------------------------------------------
        const PROGRAM_METADATA_LINK_SECTION: &str = "turbo_os_program_metadata";
        let program_metadata_string = serde_json::to_string(&program_metadata)
            .expect("Could not serialize TurboProgramMetadata ");
        let program_metadata_string = format!("{program_metadata_string}\n");
        let program_metadata_bytes = program_metadata_string
            .as_bytes()
            .iter()
            .map(|b| quote! { #b });
        let program_metadata_len = program_metadata_bytes.len();
        let program_metadata_ident = format_ident!("turbo_os_program_metadata_{}", program_name);

        let mod_inject = quote! {
            #[used]
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            #[link_section = #PROGRAM_METADATA_LINK_SECTION]
            pub static #program_metadata_ident: [u8; #program_metadata_len] = [#(#program_metadata_bytes),*];

            pub const PROGRAM_NAME: &str = #program_name;
            pub const PROGRAM_OWNER: &str = #user_id;
            pub const PROGRAM_ID: &str = #program_id;

            pub fn watch<T: turbo::borsh::BorshDeserialize>(key: impl AsRef<std::path::Path>) -> Option<T> {
                let path = std::path::Path::new(#program_id).join(key.as_ref());
                turbo::os::client::fs::watch(path).parse()
            }

            #[cfg(turbo_no_run)]
            mod program_utils {
                // Logs the incoming input (parsed via Borsh) as pretty JSON
                pub fn log_input_as_json<T: turbo::borsh::BorshDeserialize + turbo::serde::Serialize>() {
                    use turbo::borsh::BorshDeserialize;
                    use turbo::serde_json::json;
                    let bytes = turbo::os::server::command::read_input();
                    if bytes.is_empty() {
                        return turbo::os::server::log!("null");
                    }
                    let data = match T::try_from_slice(&bytes) {
                        Ok(data) => data,
                        Err(err) => return turbo::os::server::log!("{:#?}", err),
                    };
                    let json = json!(data);
                    turbo::os::server::log!("{}", json)
                }
            }
        };

        // Inject mod-level constants and helpers at the top of the module
        let parsed_items = syn::parse2::<syn::File>(mod_inject)
            .expect("failed to parse mod_inject block")
            .items;
        items.splice(0..0, parsed_items);
    }

    // Return the modified module
    quote! { #module }.into()
}

fn process_program_command(
    program_metadata: &mut TurboProgramMetadata,
    args: CommandArgs,
    item: &Item,
    ident: &Ident,
) -> Vec<Item> {
    let program_id = program_metadata.program_id.as_str();
    let program_name = program_metadata.name.as_str();
    let name = args.name;

    // Update command metadata
    program_metadata
        .commands
        .push(TurboProgramCommandMetadata { name: name.clone() });

    // Construct export identifiers and metadata symbols
    let handler_export = format!("turbo_program:command_handler/{}/{}", program_id, name);
    let handler_extern = format_ident!("command_handler_{}_{}", program_name, name);
    let de_input_export = format!("turbo_program:de_command_input/{}/{}", program_id, name);
    let de_input_extern = format_ident!("de_command_input_{}_{}", program_name, name);

    // Command registration metadata embedded .turbo_programs
    let metadata_string = format!("{program_name}/commands/{name},");
    let metadata_bytes = metadata_string.as_bytes().iter().map(|b| quote! { #b });
    let metadata_len = metadata_bytes.len();
    let metadata_ident = format_ident!("command_metadata_{}_{}", program_name, name);

    // Expand command wrapper
    let expanded = quote! {
        #[used]
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        #[link_section = "turbo_programs"]
        pub static #metadata_ident: [u8; #metadata_len] = [#(#metadata_bytes),*];

        #[turbo::serialize]
        #item

        impl #ident {
            pub fn exec(self) -> String {
                turbo::os::client::command::exec(#program_id, #name, self)
            }
        }

        #[cfg(turbo_no_run)]
        #[unsafe(export_name = #handler_export)]
        pub unsafe extern "C" fn #handler_extern() -> usize {
            let user_id = turbo::os::server::command::user_id();
            let mut cmd = turbo::os::server::command::parse_input::<#ident>();
            match &mut cmd {
                Ok(cmd) => match cmd.run(&user_id) {
                    Ok(_) => turbo::os::server::command::COMMIT,
                    Err(_) => turbo::os::server::command::CANCEL,
                },
                Err(_) => turbo::os::server::command::CANCEL,
            }
        }

        #[cfg(turbo_no_run)]
        #[unsafe(export_name = #de_input_export)]
        pub unsafe extern "C" fn #de_input_extern() {
            program_utils::log_input_as_json::<#ident>()
        }
    };

    syn::parse2::<syn::File>(expanded).unwrap().items
}

fn process_program_channel(
    program_metadata: &mut TurboProgramMetadata,
    args: ChannelArgs,
    item: &Item,
    ident: &Ident,
) -> Vec<Item> {
    let program_id = program_metadata.program_id.as_str();
    let program_name = program_metadata.name.as_str();
    let name = args.name;

    // Update channel metadata
    program_metadata
        .channels
        .push(TurboProgramChannelMetadata { name: name.clone() });

    // Build all export symbols and metadata identifiers
    let handler_export = format!("turbo_program:channel_handler/{}/{}", program_id, name);
    let handler_extern = format_ident!("channel_handler_{}_{}", program_name, name);
    let de_send_export = format!("turbo_program:de_channel_send/{}/{}", program_id, name);
    let de_send_extern = format_ident!("de_channel_send_{}_{}", program_name, name);
    let de_recv_export = format!("turbo_program:de_channel_recv/{}/{}", program_id, name);
    let de_recv_extern = format_ident!("de_channel_recv_{}_{}", program_name, name);
    let metadata_string = format!("{program_name}/channels/{name},");
    let metadata_bytes = metadata_string.as_bytes().iter().map(|b| quote! { #b });
    let metadata_len = metadata_bytes.len();
    let metadata_ident = format_ident!("channel_metadata_{}_{}", program_name, name);

    // Expand channel binding
    let expanded = quote! {
        #[used]
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        #[link_section = "turbo_programs"]
        pub static #metadata_ident: [u8; #metadata_len] = [#(#metadata_bytes),*];

        #[turbo::serialize]
        #item

        impl #ident {
            pub fn subscribe(channel_id: &str) -> Option<turbo::os::client::channel::ChannelConnection<
                <Self as turbo::os::server::channel::ChannelHandler>::Recv,
                <Self as turbo::os::server::channel::ChannelHandler>::Send,
            >> {
                turbo::os::client::channel::Channel::<
                    <Self as turbo::os::server::channel::ChannelHandler>::Recv,
                    <Self as turbo::os::server::channel::ChannelHandler>::Send,
                >::subscribe(PROGRAM_ID, #name, channel_id)
            }
        }

        #[cfg(turbo_no_run)]
        #[unsafe(export_name = #handler_export)]
        pub unsafe extern "C" fn #handler_extern() {
            use turbo::os::server::channel::{
                ChannelSettings,
                ChannelMessage,
                ChannelError,
                recv_with_timeout,
            };
            let handler = &mut #ident::new();
            let settings = &mut ChannelSettings::default();
            handler.on_open(settings);
            let timeout = settings.interval.unwrap_or(u32::MAX).max(16);
            loop {
                match recv_with_timeout(timeout) {
                    Ok(ChannelMessage::Connect(user_id, _)) => {
                        handler.on_connect(&user_id);
                    },
                    Ok(ChannelMessage::Disconnect(user_id, _)) => {
                        handler.on_disconnect(&user_id);
                    },
                    Ok(ChannelMessage::Data(user_id, data)) => match #ident::parse(&data) {
                        Ok(data) => handler.on_data(&user_id, data),
                        Err(err) => turbo::os::server::log!("Error parsing data from user {user_id}: {err:?}"),
                    },
                    Err(ChannelError::Timeout) => {
                        handler.on_interval()
                    },
                    Err(_) => {
                        handler.on_close();
                        return;
                    },
                }
            }
        }

        #[cfg(turbo_no_run)]
        #[unsafe(export_name = #de_send_export)]
        pub unsafe extern "C" fn #de_send_extern() {
            program_utils::log_input_as_json::<<#ident as turbo::os::server::channel::ChannelHandler>::Send>()
        }

        #[cfg(turbo_no_run)]
        #[unsafe(export_name = #de_recv_export)]
        pub unsafe extern "C" fn #de_recv_extern() {
            program_utils::log_input_as_json::<<#ident as turbo::os::server::channel::ChannelHandler>::Recv>()
        }
    };

    syn::parse2::<syn::File>(expanded).unwrap().items
}
