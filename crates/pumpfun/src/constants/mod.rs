//! Constants used by the crate.
//!
//! This module contains various constants used throughout the crate, including:
//!
//! - Seeds for deriving Program Derived Addresses (PDAs)
//! - Program account addresses and public keys
//!
//! The constants are organized into submodules for better organization:
//!
//! - `seeds`: Contains seed values used for PDA derivation
//! - `accounts`: Contains important program account addresses

/// Constants used as seeds for deriving PDAs (Program Derived Addresses)
pub mod seeds {
    /// Seed for the global state PDA
    pub const GLOBAL_SEED: &[u8] = b"global";

    /// Seed for the mint authority PDA
    pub const MINT_AUTHORITY_SEED: &[u8] = b"mint-authority";

    /// Seed for bonding curve PDAs
    pub const BONDING_CURVE_SEED: &[u8] = b"bonding-curve";

    /// Seed for metadata PDAs
    pub const METADATA_SEED: &[u8] = b"metadata";
}

/// Constants related to program accounts and authorities
pub mod accounts {
    use anchor_client::solana_sdk::pubkey::Pubkey;

    /// Authority for program events
    pub const EVENT_AUTHORITY: Pubkey = Pubkey::new_from_array([
        172u8, 241u8, 54u8, 235u8, 1u8, 252u8, 28u8, 78u8, 136u8, 61u8, 35u8, 200u8, 181u8, 132u8,
        74u8, 181u8, 154u8, 55u8, 246u8, 106u8, 221u8, 87u8, 197u8, 233u8, 172u8, 59u8, 83u8,
        224u8, 89u8, 211u8, 92u8, 100u8,
    ]);
}
