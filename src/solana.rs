// Imports
use super::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

// Re-exporting the `solana_sdk` crate
pub use solana_sdk;

// Function to sign and send a transaction using FFI
pub fn user_pubkey() -> Pubkey {
    unsafe {
        #[link(wasm_import_module = "@turbo_genesis/solana")]
        extern "C" {
            fn solana_user(ptr: *mut u8) -> u32;
        }
        static mut SOL_USER_PUBKEY: (bool, [u8; 32]) = (false, [0; 32]);
        if !SOL_USER_PUBKEY.0 {
            let mut pubkey_array = [0u8; 32];
            let ptr = pubkey_array.as_mut_ptr();
            if solana_user(ptr) == 0 {
                SOL_USER_PUBKEY.1 = pubkey_array;
            }
            SOL_USER_PUBKEY.0 = true;
        }
        Pubkey::new_from_array(SOL_USER_PUBKEY.1)
    }
}

// RPC module for Solana related operations
pub mod rpc {
    use super::*;
    use solana_sdk::transaction::Transaction;

    // Account information structure for Solana accounts
    #[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
    pub struct AccountInfo {
        pub data: Vec<u8>,
        pub executable: bool,
        pub lamports: u64,
        pub owner: String,
        pub rent_epoch: u64,
        pub space: u32,
    }

    // Decode account data using the Borsh deserialization method
    impl AccountInfo {
        pub fn decode_anchor<T: BorshDeserialize>(&self) -> Result<T, std::io::Error> {
            T::try_from_slice(if self.data.len() >= 8 {
                &self.data[8..]
            } else {
                &[]
            })
        }
    }

    // Enum to represent the status of a request
    #[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
    pub enum RequestStatus {
        Idle,
        Pending,
        Done,
    }

    // Context for a query result, including the slot number
    #[derive(Debug, Copy, Clone, BorshSerialize, BorshDeserialize)]
    pub struct QueryResultContext {
        pub slot: Option<u64>,
    }

    // Structure representing the result of a query to the Solana network
    #[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
    pub struct QueryResult<T, E> {
        pub context: QueryResultContext,
        pub status: RequestStatus,
        pub error: Option<E>,
        pub value: Option<T>,
    }

    // Implementation of the QueryResult struct, defining its constructor and methods
    impl<T, E> QueryResult<T, E> {
        pub fn new() -> Self {
            Self {
                context: QueryResultContext { slot: None },
                status: RequestStatus::Idle,
                error: None,
                value: None,
            }
        }

        // Checks if the query result status is 'Done'
        pub fn is_fetched(&self) -> bool {
            self.status == RequestStatus::Done
        }
    }

    // Implementation of the decode_anchor method for AccountInfo wrapped in a QueryResult
    impl<E> QueryResult<AccountInfo, E> {
        pub fn decode_anchor<T: BorshDeserialize>(&self) -> Result<T, std::io::Error> {
            if self.status == RequestStatus::Done {
                if self.value.is_none() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "No account data found",
                    ));
                }
                return self.value.as_ref().unwrap().decode_anchor::<T>();
            }
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Loading account data",
            ));
        }
    }

    // Function to sign and send a transaction using FFI
    pub fn sign_and_send_transaction(tx: &Transaction) -> bool {
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/solana")]
            extern "C" {
                fn solana_sign_and_send_transaction(tx_ptr: *const u8, tx_len: u32) -> u32;
            }
            let message_data = tx.message_data();
            let tx_ptr = message_data.as_ptr();
            let tx_len = message_data.len() as u32;
            solana_sign_and_send_transaction(tx_ptr, tx_len) == 1
        }
    }

    // Function to query an account on the Solana network
    pub fn get_account<T: ToString>(pubkey: T) -> QueryResult<AccountInfo, String> {
        let pubkey = pubkey.to_string();
        let pubkey = pubkey.as_str();
        unsafe {
            #[link(wasm_import_module = "@turbo_genesis/solana")]
            extern "C" {
                fn solana_get_account(
                    pk_ptr: *const u8,
                    pk_len: u32,
                    data_ptr: *mut u8,
                    data_len_ptr: *mut u32,
                    cache_status: u32, // 0 - None, 1 - Idle, 2 - Pending, 3 - Done
                    cache_slot: u64,
                ) -> u32;
            }

            static mut SOLANA_ACCOUNT_QUERY_CACHE: Option<
                HashMap<String, QueryResult<AccountInfo, String>>,
            > = None;
            if SOLANA_ACCOUNT_QUERY_CACHE.is_none() {
                SOLANA_ACCOUNT_QUERY_CACHE = Some(HashMap::new());
            }
            let cache = SOLANA_ACCOUNT_QUERY_CACHE.as_mut().unwrap();
            let prev = cache.get(pubkey);
            let (cache_status, cache_slot) = prev
                .map(|res| {
                    let status: u32 = match res.status {
                        RequestStatus::Idle => 1,
                        RequestStatus::Pending => 2,
                        RequestStatus::Done => 3,
                    };
                    let slot = res.context.slot.unwrap_or(0);
                    (status, slot)
                })
                .unwrap_or((0, 0));

            let pk_ptr = pubkey.as_ptr();
            let pk_len = pubkey.len() as u32;
            let mut data = vec![0; 2048];
            let mut data_len: u32 = 0;
            if 0 == solana_get_account(
                pk_ptr,
                pk_len,
                data.as_mut_ptr(),
                &mut data_len,
                cache_status,
                cache_slot,
            ) {
                return cache
                    .entry(pubkey.to_string())
                    .or_insert(QueryResult::new())
                    .clone();
            }
            match <QueryResult<AccountInfo, String>>::try_from_slice(&data[..data_len as usize]) {
                Ok(data) => {
                    // crate::println!("UPDATING CACHE: {:?}", data);
                    cache.insert(pubkey.to_string(), data);
                }
                Err(_err) => {
                    // crate::println!("ERR: {:?}", err);
                    cache.insert(pubkey.to_string(), QueryResult::new());
                }
            }
            cache.get(pubkey).unwrap().clone()
        }
    }
}

// Anchor module for handling Solana Anchor program interactions
pub mod anchor {
    // Importing necessary modules and traits from super and solana_sdk
    use super::*;
    use solana_sdk::{
        hash::Hash,
        instruction::{AccountMeta, Instruction as SolanaInstruction},
        pubkey::Pubkey,
        transaction::Transaction,
    };
    use std::ops::Deref;

    // Function to deserialize data using Borsh deserialization, skipping the first 8 bytes if possible
    pub fn try_from_slice<T: BorshDeserialize>(data: &[u8]) -> Result<T, std::io::Error> {
        T::try_from_slice(if data.len() >= 8 { &data[8..] } else { &[] })
    }

    // Structure representing a Solana program with its program ID
    #[derive(Debug, Clone)]
    pub struct Program {
        program_id: Pubkey,
    }

    // Implementation of methods for the Program struct
    impl Program {
        // Constructor to create a new Program instance with a given program ID
        pub fn new(program_id: Pubkey) -> Self {
            Self { program_id }
        }

        // Method to create an Instruction associated with this program
        pub fn instruction(&self, name: &str) -> Instruction {
            Instruction::new(self.program_id, name)
        }
    }

    // Structure representing an instruction to be sent to a Solana program
    #[derive(Debug, Clone)]
    pub struct Instruction {
        program_id: Pubkey,
        name: String,
        accounts: Vec<AccountMeta>,
        data: Vec<u8>,
    }

    // Implementation of methods for the Instruction struct
    impl Instruction {
        // Constructor to create a new Instruction instance
        pub fn new(program_id: Pubkey, name: &str) -> Self {
            Self {
                program_id,
                name: format!("global:{}", name),
                accounts: vec![],
                data: vec![],
            }
        }

        // Method to set the accounts associated with this instruction
        pub fn accounts(mut self, accounts: Vec<AccountMeta>) -> Self {
            self.accounts = accounts;
            self
        }

        // Method to set raw data for this instruction
        pub fn data(mut self, data: Vec<u8>) -> Self {
            self.data = data;
            self
        }

        // Method to serialize arguments using Borsh and set as instruction data
        pub fn args<T: BorshSerialize>(mut self, data: T) -> Self {
            self.data = data.try_to_vec().unwrap();
            self
        }

        // Converts this Instruction into a Solana SDK Instruction
        pub fn instruction(&self) -> SolanaInstruction {
            SolanaInstruction {
                program_id: self.program_id.clone(),
                accounts: self.accounts.clone(),
                data: {
                    let mut sighash = [0; 8];
                    sighash.copy_from_slice(
                        &solana_sdk::hash::hash(self.name.as_bytes()).to_bytes()[..8],
                    );
                    let len = (self.data.len() as u32).to_le_bytes();
                    [sighash.as_ref(), len.as_ref(), self.data.clone().deref()].concat()
                },
            }
        }

        // Creates a new transaction with this instruction and no specified payer
        pub fn transaction(&self) -> Transaction {
            Transaction::new_with_payer(&[self.instruction()], None)
        }

        // Sends the transaction, signs, and sends it to the network, returning its hash
        pub fn send(&self) -> Hash {
            let tx = self.transaction();
            rpc::sign_and_send_transaction(&tx);
            tx.message().hash()
        }
    }
}
