//! Turbo Genesis Macros
//!
//! This module implements the core procedural macros used by Turbo Genesis:
//!
//! - `#[serialize]`: Automatically derive Borsh and Serde traits.
//! - `#[game]`: Generate entry points for hot-reload and release game loops.
//! - `#[command]`: Embed command metadata, derive serialization, and export FFI bindings.
//! - `#[channel]`: Embed channel metadata, derive serialization, and export subscription bindings.
//! - `#[document]`: Derive serialization and implement `HasProgramId` for document types.
//!
//! Utilities within include:
//! - Argument parsers (`CommandArgs`, `ChannelArgs`).
//! - Helpers to inline modules and compute project metadata from `Cargo.toml`.

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    Error, Item, ItemEnum, ItemStruct, LitStr,
};
use turbo_genesis_abi::{
    TurboProgramChannelMetadata, TurboProgramCommandMetadata, TurboProgramMetadata,
};

// =============================================================================
// Helpers
// =============================================================================

/// Retrieves the project’s root directory from the `CARGO_MANIFEST_DIR` env var.
///
/// Panics if the environment variable is empty or not set.
fn get_project_path() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    assert!(
        !manifest_dir.is_empty(),
        "CARGO_MANIFEST_DIR unavailable. Could not determine project directory."
    );
    Path::new(&manifest_dir).to_path_buf()
}

/// Reads the user’s UUID from `[package.metadata.turbo].user` in Cargo.toml
/// and combines it with `program_name` to produce a unique, stable `program_id`.
///
/// # Steps
/// 1. Load and parse `Cargo.toml`.  
/// 2. Extract the `user` field under `[package.metadata.turbo]`.  
/// 3. Compute SHA-256 hash over the UUID bytes and `program_name`.  
/// 4. Base64-url-encode the hash to produce `program_id`.
fn create_program_metadata(program_name: &str) -> TurboProgramMetadata {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD as b64_url_safe, Engine};
    use sha2::{Digest, Sha256};

    // Locate and read Cargo.toml
    let project_dir = get_project_path();
    let cargo_toml_path = project_dir.join("Cargo.toml");
    let cargo_toml = std::fs::read_to_string(&cargo_toml_path)
        .unwrap_or_else(|e| panic!("Could not read Cargo.toml: {e:?}"));

    // Parse TOML and extract user UUID
    let parsed: toml_edit::DocumentMut = cargo_toml
        .parse()
        .unwrap_or_else(|e| panic!("Invalid Cargo.toml syntax: {e:?}"));
    let user_id = parsed["package"]["metadata"]["turbo"]["user"]
        .as_str()
        .expect("Missing [package.metadata.turbo].user entry");

    // Hash UUID + program name
    let uuid = uuid::Uuid::parse_str(user_id).expect("Invalid UUID format");
    let mut hasher = Sha256::new();
    hasher.update(uuid.as_bytes());
    hasher.update(program_name.as_bytes());
    let program_id = b64_url_safe.encode(hasher.finalize());

    TurboProgramMetadata {
        name: program_name.to_string(),
        program_id,
        owner_id: user_id.to_string(),
        commands: BTreeSet::new(),
        channels: BTreeSet::new(),
    }
}

// =============================================================================
// Serialize Macro
// =============================================================================

/// Derive Borsh and Serde serialization for structs and enums.
///
/// Expands to:
/// - `#[derive(Debug, Clone, BorshDeserialize, BorshSerialize, Deserialize, Serialize)]`
/// - Configures `#[borsh(crate = "turbo::borsh")]` and `#[serde(crate = "turbo::serde")]`.
///
/// # Usage
/// ```ignore
/// #[turbo::serialize]
/// struct MyType { /* fields */ }
/// ```
#[proc_macro_attribute]
pub fn serialize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input token stream as a syn Item (struct, enum, etc.)
    let input = parse_macro_input!(item as Item);

    // Only allow structs and enums
    match &input {
        Item::Struct(_) | Item::Enum(_) => (),
        _ => {
            // Emit a compile error if used on an unsupported item
            return quote! {
                compile_error!("#[turbo::serialize] only supports structs and enums.");
            }
            .into();
        }
    };

    // Expand to derive the necessary traits and set crate paths
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
// Game Macro
// =============================================================================

/// Procedural macro for defining a Turbo game entry point.
///
/// Injects a `run()` function that:
/// - On hot-reload, deserializes previous state or creates new state.
/// - Calls `state.update()` and `turbo::camera::update()`.
/// - Persists state via Borsh back into host.
///
/// # Usage
/// ```ignore
/// #[turbo::game]
/// struct GameState { /* ... */ }
/// ```
#[proc_macro_attribute]
pub fn game(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    let ident = match &item {
        Item::Struct(ItemStruct { ident, .. }) | Item::Enum(ItemEnum { ident, .. }) => ident,
        _ => {
            return quote! {
                compile_error!("#[turbo::game] only supports structs and enums.");
            }
            .into();
        }
    };

    quote! {
        #[turbo::serialize]
        #item

        #[no_mangle]
        #[cfg(all(turbo_hot_reload, not(turbo_no_run)))]
        pub unsafe extern "C" fn run() {
            use turbo::borsh::*;
            let mut state = match hot::load() {
                Ok(bytes) => <#ident>::try_from_slice(&bytes).unwrap_or_else(|_| #ident::new()),
                Err(err) => {
                    log!("[turbo] Hot reload deserialization failed: {err:?}");
                    #ident::new()
                },
            };
            state.update();
            turbo::camera::update();
            if let Ok(bytes) = borsh::to_vec(&state) {
                if let Err(err) = hot::save(&bytes) {
                    log!("[turbo] hot save failed: Error code {err}");
                }
            }
        }

        #[no_mangle]
        #[cfg(all(not(turbo_hot_reload), not(turbo_no_run)))]
        pub unsafe extern "C" fn run() {
            static mut GAME_STATE: Option<#ident> = None;
            let mut state = GAME_STATE.take().unwrap_or_else(|| #ident::new());
            state.update();
            turbo::camera::update();
            GAME_STATE = Some(state);
        }

        #[doc(hidden)]
        #[cfg(turbo_no_run)]
        pub(crate) mod __turbo_os_program_utils {
            // Logs the incoming input (parsed via Borsh) as pretty JSON
            pub fn log_input_as_json<T: turbo::borsh::BorshDeserialize + turbo::serde::Serialize>() {
                use turbo::borsh::BorshDeserialize;
                use turbo::serde_json::json;
                let bytes = turbo::os::server::command::read_input();
                if bytes.is_empty() {
                    return turbo::log!("null");
                }
                let data = match T::try_from_slice(&bytes) {
                    Ok(data) => data,
                    Err(err) => return turbo::log!("{}", json!({ "error": err.to_string(), "input": bytes })),
                };
                let json = json!(data);
                turbo::log!("{}", json)
            }
        }
    }.into()
}

// =============================================================================
// Command Macro
// =============================================================================

/// Parsed arguments for `#[command(program = "...", name = "...")]`.
#[derive(Debug, Clone, Default)]
struct CommandArgs {
    /// A program name
    pub program: String,
    /// A command name
    pub name: String,
}
impl Parse for CommandArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            let _: syn::Token![=] = input.parse()?;
            let value: LitStr = input.parse()?;
            match ident.to_string().as_str() {
                "program" => args.program = value.value(),
                "name" => args.name = value.value(),
                _ => return Err(syn::Error::new_spanned(ident, "unexpected attribute key")),
            }
            if input.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
        }
        if args.program.is_empty() {
            return Err(input.error("missing `program`"));
        }
        if args.name.is_empty() {
            return Err(input.error("missing `name`"));
        }
        Ok(args)
    }
}

/// Attribute macro `#[command]` for generating FFI bindings and client helpers
/// on a struct or enum that implements `CommandHandler`.
///
/// # Parameters
/// - `attr`: TokenStream of attribute arguments (e.g. `program = "foo", name = "bar"`).
/// - `item`: TokenStream of the annotated item (must be a `struct` or `enum`).
///
/// # Behavior
/// 1. Parses `item` into a `syn::Item`.
/// 2. Parses `attr` into our `CommandArgs` helper (extracting `program` and `name`).
/// 3. If `item` is a `struct` or `enum`, obtains its identifier (`ident`), builds/upgrades
///    the program metadata via `create_program_metadata(&args.program)`, and calls
///    `process_program_command(...)` to emit the expanded FFI binding and metadata embedding.
/// 4. If used on any other item, emits a compile error pointing at the original span,
///    instructing that `#[command]` only applies to types implementing `CommandHandler`.
#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Step 1: Parse the annotated item (must be struct or enum)
    let item = parse_macro_input!(item as Item);

    // Step 2: Parse the attribute arguments into our helper struct
    let args = parse_macro_input!(attr as CommandArgs);

    // Step 3: Dispatch on item type
    match &item {
        Item::Struct(ItemStruct { ident, .. }) | Item::Enum(ItemEnum { ident, .. }) => {
            // Build or update the program metadata from the `program` argument
            let mut program_metadata = create_program_metadata(&args.program);
            // Generate and return the expanded code (FFI exports, exec helper, metadata)
            process_program_command(&mut program_metadata, args.clone(), &item, ident)
        }
        // Unsupported item: error at original location
        _ => Error::new(
            item.span(),
            "`#[command]` may only be used on a struct or enum that implements `CommandHandler`",
        )
        .to_compile_error()
        .into(),
    }
}

/// Generates the expanded code for a `#[command]`-annotated struct or enum.
///
/// This function embeds metadata, derives serialization, and creates both
/// client-side execution helpers and server-side FFI exports.
///
/// # Parameters
/// - `program_metadata`: Mutable reference to the program’s metadata record.  
///   This will be updated with the new command’s name.
/// - `args`: The parsed `program` and `name` arguments from the attribute.  
/// - `item`: The original `syn::Item` (struct or enum) to which the macro is applied.
/// - `ident`: The identifier of the struct or enum being processed.
///
/// # Returns
/// A `TokenStream` containing:
/// 1. A `static` byte array in the `.turbo_programs` section with updated metadata.  
/// 2. The original type, annotated with `#[turbo::serialize]`.  
/// 3. An inherent `impl` block on the type, adding:
///    - `PROGRAM_ID` and `PROGRAM_OWNER` constants.  
///    - An `exec(self) -> String` method for client-side invocation.  
/// 4. Under `#[cfg(turbo_no_run)]`, two `extern "C"` functions:
///    - The command handler entrypoint (returns commit/cancel codes).  
///    - A de-input hook for logging JSON on the server.
fn process_program_command(
    program_metadata: &mut TurboProgramMetadata,
    args: CommandArgs,
    item: &Item,
    ident: &Ident,
) -> TokenStream {
    // Extract basic identifiers and names
    let program_id = program_metadata.program_id.as_str();
    let program_name = program_metadata.name.as_str();
    let owner_id = program_metadata.owner_id.as_str();
    let name = args.name;

    // 1) Register this command in the program’s metadata
    program_metadata
        .commands
        .insert(TurboProgramCommandMetadata { name: name.clone() });

    // 2) Build the symbols used for linking and exports
    let handler_export = format!("turbo_program:command_handler/{}/{}", program_id, name);
    let handler_extern = format_ident!("command_handler_{}_{}", program_name, name);
    let de_input_export = format!("turbo_program:de_command_input/{}/{}", program_id, name);
    let de_input_extern = format_ident!("de_command_input_{}_{}", program_name, name);

    // 3) Serialize the updated metadata to JSON and embed it
    let metadata_string = serde_json::to_string(program_metadata).unwrap() + "\n";
    let metadata_bytes = metadata_string.as_bytes().iter().map(|b| quote! { #b });
    let metadata_len = metadata_bytes.len();
    let metadata_ident = format_ident!(
        "turbo_os_program_metadata_command_{}_{}",
        program_name,
        name
    );

    // 4) Emit the final expanded code
    quote! {
        // a) Embed the JSON metadata as a static byte array
        #[used]
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        #[link_section = "turbo_os_program_metadata"]
        pub static #metadata_ident: [u8; #metadata_len] = [#(#metadata_bytes),*];

        // b) Derive serialization on the user’s type
        #[turbo::serialize]
        #item

        // c) Implement client-side helpers on the type
        impl #ident {
            /// The program’s unique ID constant.
            pub const PROGRAM_ID: &'static str = #program_id;
            /// The program’s owner UUID string.
            pub const PROGRAM_OWNER: &'static str = #owner_id;

            /// Execute this command on the client, returning the host’s response.
            pub fn exec(self) -> String {
                turbo::os::client::command::exec(#program_id, #name, self)
            }
        }

        // d) Server-side FFI entrypoint for the command handler
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

        // e) Server-side de-input hook for logging raw JSON
        #[cfg(turbo_no_run)]
        #[unsafe(export_name = #de_input_export)]
        pub unsafe extern "C" fn #de_input_extern() {
            crate::__turbo_os_program_utils::log_input_as_json::<#ident>()
        }
    }
    .into()
}

// =============================================================================
// Channel Macro
// =============================================================================

/// Parsed arguments for `#[channel(program = "...", name = "...")]`.
#[derive(Debug, Clone, Default)]
struct ChannelArgs {
    /// A program name
    pub program: String,
    /// A channel name
    pub name: String,
}
impl Parse for ChannelArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            let _: syn::Token![=] = input.parse()?;
            let value: LitStr = input.parse()?;
            match ident.to_string().as_str() {
                "program" => args.program = value.value(),
                "name" => args.name = value.value(),
                _ => return Err(syn::Error::new_spanned(ident, "unexpected attribute key")),
            }
            if input.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
        }
        if args.program.is_empty() {
            return Err(input.error("missing `program`"));
        }
        if args.name.is_empty() {
            return Err(input.error("missing `name`"));
        }
        Ok(args)
    }
}

/// Attribute macro `#[channel]` for generating client and server bindings
/// on a struct or enum that implements `ChannelHandler`.
///
/// # Parameters
/// - `attr`: The attribute arguments as a TokenStream (e.g. `program = "foo", name = "bar"`).
/// - `item`: The annotated item’s TokenStream (must be a `struct` or `enum`).
///
/// # Behavior
/// 1. Parses the annotated item into a `syn::Item`.
/// 2. Parses `attr` into our `ChannelArgs` (extracting `program` and `name`).
/// 3. Matches on `item`:
///    - If it’s a `struct` or `enum`, extracts its identifier (`ident`).
///    - Calls `create_program_metadata(&args.program)` to build or update metadata.
///    - Invokes `process_program_channel(...)` to generate the FFI bindings and
///      subscription helper, returning that expanded TokenStream directly.
/// 4. If used on any other item, emits a compile error pointing at the original span,
///    instructing that `#[channel]` only applies to types implementing `ChannelHandler`.
#[proc_macro_attribute]
pub fn channel(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Step 1: Parse the annotated item (struct or enum)
    let item = parse_macro_input!(item as Item);

    // Step 2: Parse the attribute arguments into our helper struct
    let args = parse_macro_input!(attr as ChannelArgs);

    // Step 3: Ensure we only operate on structs or enums
    match &item {
        Item::Struct(ItemStruct { ident, .. }) | Item::Enum(ItemEnum { ident, .. }) => {
            // Build or update the program metadata from the `program` argument
            let mut program_metadata = create_program_metadata(&args.program);
            // Generate the expanded code (FFI exports, subscription API, metadata embedding)
            let expanded =
                process_program_channel(&mut program_metadata, args.clone(), &item, ident);
            // Return the generated code
            return expanded;
        }
        // Anything else is unsupported: emit a clear compile-time error
        _ => Error::new(
            item.span(),
            "`#[channel]` may only be used on a struct or enum that implements `ChannelHandler`",
        )
        .to_compile_error()
        .into(),
    }
}

/// Generates the expanded code for a `#[channel]`-annotated struct or enum.
///
/// Embeds metadata, derives serialization, and produces both client subscription
/// helpers and server-side FFI exports.
///
/// # Parameters
/// - `program_metadata`: Mutable reference to the program’s metadata record.  
///   This will be updated with the new channel’s name.
/// - `args`: The parsed `program` and `name` arguments from the attribute.  
/// - `item`: The original `syn::Item` (struct or enum) to which the macro is applied.
/// - `ident`: The identifier of the struct or enum being processed.
///
/// # Returns
/// A `TokenStream` containing:
/// 1. A `static` byte array with channel metadata.  
/// 2. The original type, annotated with `#[turbo::serialize]`.  
/// 3. An inherent `impl` block adding:
///    - A `subscribe(channel_id) -> Option<Connection>` method for clients.  
/// 4. Under `#[cfg(turbo_no_run)]`, `extern "C"` functions for server-side:
///    - The channel handler (open/connect/data/timeout/close loop).  
///    - De-send and de-receive hooks for logging.
fn process_program_channel(
    program_metadata: &mut TurboProgramMetadata,
    args: ChannelArgs,
    item: &Item,
    ident: &Ident,
) -> TokenStream {
    // Extract identifiers
    let program_id = program_metadata.program_id.as_str();
    let program_name = program_metadata.name.as_str();
    let name = args.name;

    // 1) Register this channel in the program’s metadata
    program_metadata
        .channels
        .insert(TurboProgramChannelMetadata { name: name.clone() });

    // 2) Build linking symbols for subscribe and handlers
    let handler_export = format!("turbo_program:channel_handler/{}/{}", program_id, name);
    let handler_extern = format_ident!("channel_handler_{}_{}", program_name, name);
    let de_send_export = format!("turbo_program:de_channel_send/{}/{}", program_id, name);
    let de_send_extern = format_ident!("de_channel_send_{}_{}", program_name, name);
    let de_recv_export = format!("turbo_program:de_channel_recv/{}/{}", program_id, name);
    let de_recv_extern = format_ident!("de_channel_recv_{}_{}", program_name, name);

    // 3) Serialize and embed updated channel metadata
    let metadata_string = serde_json::to_string(program_metadata).unwrap() + "\n";
    let metadata_bytes = metadata_string.as_bytes().iter().map(|b| quote! { #b });
    let metadata_len = metadata_bytes.len();
    let metadata_ident = format_ident!(
        "turbo_os_program_metadata_channel_{}_{}",
        program_name,
        name
    );

    // 4) Emit final expanded code
    quote! {
        // a) Embed channel metadata
        #[used]
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        #[link_section = "turbo_os_program_metadata"]
        pub static #metadata_ident: [u8; #metadata_len] = [#(#metadata_bytes),*];

        // b) Derive serialization on the type
        #[turbo::serialize]
        #item

        // c) Client subscription helper
        impl #ident {
            /// Subscribe to this channel ID, returning a connection if available.
            pub fn subscribe(
                channel_id: &str
            ) -> Option<turbo::os::client::channel::ChannelConnection<
                <Self as turbo::os::server::channel::ChannelHandler>::Recv,
                <Self as turbo::os::server::channel::ChannelHandler>::Send,
            >> {
                turbo::os::client::channel::Channel::subscribe(#program_id, #name, channel_id)
            }
        }

        // d) Server-side channel handler loop
        #[cfg(turbo_no_run)]
        #[unsafe(export_name = #handler_export)]
        pub unsafe extern "C" fn #handler_extern() {
            use turbo::os::server::channel::{ChannelSettings, ChannelMessage, ChannelError, recv_with_timeout};
            let handler = &mut #ident::new();
            let settings = &mut ChannelSettings::default();

            // on_open hook
            if let Err(err) = handler.on_open(settings) {
                turbo::log!("Error in on_open: {err:?}");
                return;
            }

            // Main receive loop with timeout
            let timeout = settings.interval.unwrap_or(u32::MAX).max(16);
            loop {
                match recv_with_timeout(timeout) {
                    Ok(ChannelMessage::Connect(user_id, _)) => {
                        let _ = handler.on_connect(&user_id).map_err(|e| turbo::log!("on_connect err: {e:?}"));
                    }
                    Ok(ChannelMessage::Disconnect(user_id, _)) => {
                        let _ = handler.on_disconnect(&user_id).map_err(|e| turbo::log!("on_disconnect err: {e:?}"));
                    }
                    Ok(ChannelMessage::Data(user_id, data)) => match #ident::parse(&data) {
                        Ok(data) => {
                            let _ = handler.on_data(&user_id, data).map_err(|e| turbo::log!("on_data err: {e:?}")); 
                        }
                        Err(err) => turbo::log!("Error parsing data: {err:?}"),
                    }
                    Err(ChannelError::Timeout) => {
                        let _ = handler.on_interval().map_err(|e| turbo::log!("on_interval err: {e:?}"));  
                    }
                    Err(_) => {
                        let _ = handler.on_close().map_err(|e| turbo::log!("on_close err: {e:?}"));  
                        return;
                    }
                }
            }
        }

        // e) De-send hook: log input as JSON for outgoing messages
        #[cfg(turbo_no_run)]
        #[unsafe(export_name = #de_send_export)]
        pub unsafe extern "C" fn #de_send_extern() {
            crate::__turbo_os_program_utils::log_input_as_json::<<#ident as turbo::os::server::channel::ChannelHandler>::Send>()
        }

        // f) De-receive hook: log input as JSON for incoming messages
        #[cfg(turbo_no_run)]
        #[unsafe(export_name = #de_recv_export)]
        pub unsafe extern "C" fn #de_recv_extern() {
            crate::__turbo_os_program_utils::log_input_as_json::<<#ident as turbo::os::server::channel::ChannelHandler>::Recv>()
        }
    }.into()
}

// =============================================================================
// Document Macro
// =============================================================================

/// Parsed arguments for `#[document(program = "...")]`.
#[derive(Debug, Clone, Default)]
struct DocumentArgs {
    /// A program name
    pub program: String,
}
impl Parse for DocumentArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            let _: syn::Token![=] = input.parse()?;
            let value: LitStr = input.parse()?;
            match ident.to_string().as_str() {
                "program" => args.program = value.value(),
                _ => return Err(syn::Error::new_spanned(ident, "unexpected attribute key")),
            }
            if input.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
        }
        if args.program.is_empty() {
            return Err(input.error("missing `program`"));
        }
        Ok(args)
    }
}

/// Attribute macro `#[document]` for embedding program metadata and deriving serialization
/// on a struct or enum representing a document type.
///
/// # Parameters
/// - `attr`: TokenStream of attribute arguments (e.g. `program = "my_prog"`).
/// - `item`: TokenStream of the annotated item (must be a `struct` or `enum`).
///
/// # Behavior
/// 1. Parses `item` into a `syn::Item`.
/// 2. Parses `attr` into `DocumentArgs`, extracting the `program` identifier.
/// 3. If `item` is a `struct` or `enum`:
///    - Calls `create_program_metadata(&args.program)` to compute the stable `program_id`.
///    - Emits:
///      - `#[turbo::serialize]` on the original type to derive Borsh/Serde traits.
///      - An `impl HasProgramId` for the type, setting `PROGRAM_ID` to the computed ID.
/// 4. If used on any other item, produces a compile error at the original span.
///
/// # Example
/// ```ignore
/// #[document(program = "counter")]
/// struct Counter { /* fields */ }
/// ```
#[proc_macro_attribute]
pub fn document(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Step 1: Parse the annotated item (struct or enum)
    let item = parse_macro_input!(item as Item);

    // Step 2: Parse the attribute arguments into our helper struct
    let args = parse_macro_input!(attr as DocumentArgs);

    // Step 3: Ensure only structs or enums are supported
    match &item {
        Item::Struct(ItemStruct { ident, .. }) | Item::Enum(ItemEnum { ident, .. }) => {
            // Compute or update the program metadata (hashing UUID + program name)
            let program_metadata = create_program_metadata(&args.program);
            // Extract the base64-encoded program ID string
            let program_id = program_metadata.program_id;
            // Generate the expanded code:
            //  - Derive Borsh/Serde serialization via #[turbo::serialize]
            //  - Implement HasProgramId with the computed PROGRAM_ID constant
            quote! {
                #[turbo::serialize]
                #item

                impl turbo::os::HasProgramId for #ident {
                    const PROGRAM_ID: &'static str = #program_id;
                }
            }
            .into()
        }
        // Unsupported item types produce a clear compile-time error
        _ => Error::new(
            item.span(),
            "`#[document]` may only be used on a struct or enum representing a document type",
        )
        .to_compile_error()
        .into(),
    }
}
