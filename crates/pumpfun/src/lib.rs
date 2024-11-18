#![doc = include_str!("../RUSTDOC.md")]

pub mod accounts;
pub mod constants;
pub mod error;
pub mod utils;

use anchor_client::{
    anchor_lang::{prelude::System, Id},
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        rent::Rent,
        signature::{Keypair, Signature},
        signer::Signer,
        sysvar::SysvarId,
    },
    Client, Cluster, Program,
};
use anchor_spl::{
    associated_token::{self, get_associated_token_address},
    token,
};
use borsh::BorshDeserialize;
pub use pumpfun_cpi as cpi;
use std::rc::Rc;

/// Main client for interacting with the Pump.fun program
pub struct PumpFun<'a> {
    /// RPC client for Solana network requests
    pub rpc: RpcClient,
    /// Keypair used to sign transactions
    pub payer: &'a Keypair,
    /// Anchor client instance
    pub client: Client<Rc<&'a Keypair>>,
    /// Anchor program instance
    pub program: Program<Rc<&'a Keypair>>,
}

impl<'a> PumpFun<'a> {
    /// Creates a new PumpFun client instance
    ///
    /// # Arguments
    ///
    /// * `cluster` - Solana cluster to connect to
    /// * `payer` - Keypair used to sign transactions
    /// * `options` - Optional commitment config
    /// * `ws` - Whether to use websocket connection
    pub fn new(
        cluster: Cluster,
        payer: &'a Keypair,
        options: Option<CommitmentConfig>,
        ws: Option<bool>,
    ) -> Self {
        // Create Solana RPC Client
        let rpc: RpcClient = RpcClient::new(if ws.unwrap_or(false) {
            cluster.ws_url()
        } else {
            cluster.url()
        });

        // Create Anchor Client
        let client: Client<Rc<&Keypair>> = if let Some(options) = options {
            Client::new_with_options(cluster.clone(), Rc::new(payer), options)
        } else {
            Client::new(cluster.clone(), Rc::new(payer))
        };

        // Create Anchor Program
        let program: Program<Rc<&Keypair>> = client.program(cpi::ID).unwrap();

        // Return PumpFun struct
        Self {
            rpc,
            payer,
            client,
            program,
        }
    }

    /// Creates a new token with metadata
    ///
    /// # Arguments
    ///
    /// * `mint` - Keypair for the new token mint
    /// * `metadata` - Token metadata including name, symbol and URI
    pub async fn create(
        &self,
        mint: &Keypair,
        metadata: utils::CreateTokenMetadata,
    ) -> Result<Signature, error::ClientError> {
        let bonding_curve: Pubkey = Self::get_bonding_curve_pda(&mint.pubkey())
            .ok_or(error::ClientError::BondingCurveNotFound)?;
        let ipfs: utils::TokenMetadataResponse = utils::create_token_metadata(metadata)
            .await
            .map_err(error::ClientError::UploadMetadataError)?;

        let signature: Signature = self
            .program
            .request()
            .accounts(cpi::accounts::Create {
                associated_bonding_curve: get_associated_token_address(
                    &bonding_curve,
                    &mint.pubkey(),
                ),
                associated_token_program: associated_token::ID,
                bonding_curve,
                event_authority: constants::accounts::EVENT_AUTHORITY,
                global: Self::get_global_pda(),
                metadata: Self::get_metadata_pda(&mint.pubkey()),
                mint: mint.pubkey(),
                mint_authority: Self::get_mint_authority_pda(),
                mpl_token_metadata: constants::accounts::MPL_TOKEN_METADATA,
                program: cpi::ID,
                rent: Rent::id(),
                system_program: System::id(),
                token_program: token::ID,
                user: self.payer.pubkey(),
            })
            .args(cpi::instruction::Create {
                _name: ipfs.metadata.name,
                _symbol: ipfs.metadata.symbol,
                _uri: ipfs.metadata.image,
            })
            .signer(&self.payer)
            .signer(&mint)
            .send()
            .await
            .map_err(error::ClientError::AnchorClientError)?;

        Ok(signature)
    }

    /// Buys tokens using SOL
    ///
    /// # Arguments
    ///
    /// * `mint` - Public key of the token mint
    /// * `amount_sol` - Amount of SOL to spend in lamports
    /// * `slippage_basis_points` - Optional slippage tolerance in basis points (1 bp = 0.01%)
    pub async fn buy(
        &self,
        mint: &Pubkey,
        amount_sol: u64,
        slippage_basis_points: Option<u64>,
    ) -> Result<Signature, error::ClientError> {
        let bonding_curve =
            Self::get_bonding_curve_pda(mint).ok_or(error::ClientError::BondingCurveNotFound)?;
        let global_account = self.get_global_account()?;
        let bonding_curve_account = self.get_bonding_curve_account(mint)?;
        let buy_amount = bonding_curve_account
            .get_buy_price(amount_sol)
            .map_err(error::ClientError::BondingCurveError)?;
        let buy_amount_with_slippage =
            utils::calculate_with_slippage_buy(buy_amount, slippage_basis_points.unwrap_or(500));

        let signature: Signature = self
            .program
            .request()
            .accounts(cpi::accounts::Buy {
                associated_bonding_curve: get_associated_token_address(
                    &bonding_curve,
                    &mint.clone(),
                ),
                associated_user: get_associated_token_address(&self.payer.pubkey(), &mint.clone()),
                bonding_curve,
                event_authority: constants::accounts::EVENT_AUTHORITY,
                fee_recipient: global_account.fee_recipient,
                global: Self::get_global_pda(),
                mint: *mint,
                program: cpi::ID,
                rent: Rent::id(),
                system_program: System::id(),
                token_program: token::ID,
                user: self.payer.pubkey(),
            })
            .args(cpi::instruction::Buy {
                _amount: buy_amount,
                _max_sol_cost: buy_amount_with_slippage,
            })
            .signer(&self.payer)
            .send()
            .await
            .map_err(error::ClientError::AnchorClientError)?;

        Ok(signature)
    }

    /// Sells tokens for SOL
    ///
    /// # Arguments
    ///
    /// * `mint` - Public key of the token mint
    /// * `amount_token` - Amount of tokens to sell
    /// * `slippage_basis_points` - Optional slippage tolerance in basis points (1 bp = 0.01%)
    pub async fn sell(
        &self,
        mint: &Pubkey,
        amount_token: u64,
        slippage_basis_points: Option<u64>,
    ) -> Result<Signature, error::ClientError> {
        let bonding_curve =
            Self::get_bonding_curve_pda(mint).ok_or(error::ClientError::BondingCurveNotFound)?;
        let global_account = self.get_global_account()?;
        let bonding_curve_account = self.get_bonding_curve_account(mint)?;
        let min_sol_output = bonding_curve_account
            .get_sell_price(amount_token, global_account.fee_basis_points)
            .map_err(error::ClientError::BondingCurveError)?;
        let min_sol_output_with_slippage = utils::calculate_with_slippage_sell(
            min_sol_output,
            slippage_basis_points.unwrap_or(500),
        );

        let signature: Signature = self
            .program
            .request()
            .accounts(cpi::accounts::Sell {
                associated_bonding_curve: get_associated_token_address(
                    &bonding_curve,
                    &mint.clone(),
                ),
                associated_token_program: associated_token::ID,
                associated_user: get_associated_token_address(&self.payer.pubkey(), &mint.clone()),
                bonding_curve,
                event_authority: constants::accounts::EVENT_AUTHORITY,
                fee_recipient: global_account.fee_recipient,
                global: Self::get_global_pda(),
                mint: *mint,
                program: cpi::ID,
                system_program: System::id(),
                token_program: token::ID,
                user: self.payer.pubkey(),
            })
            .args(cpi::instruction::Sell {
                _amount: amount_token,
                _min_sol_output: min_sol_output_with_slippage,
            })
            .signer(&self.payer)
            .send()
            .await
            .map_err(error::ClientError::AnchorClientError)?;

        Ok(signature)
    }

    /// Gets the PDA for the global state account
    pub fn get_global_pda() -> Pubkey {
        let seeds: &[&[u8]; 1] = &[constants::seeds::GLOBAL_SEED];
        let program_id: &Pubkey = &cpi::ID;
        Pubkey::find_program_address(seeds, program_id).0
    }

    /// Gets the PDA for the mint authority
    pub fn get_mint_authority_pda() -> Pubkey {
        let seeds: &[&[u8]; 1] = &[constants::seeds::MINT_AUTHORITY_SEED];
        let program_id: &Pubkey = &cpi::ID;
        Pubkey::find_program_address(seeds, program_id).0
    }

    /// Gets the PDA for a token's bonding curve account
    pub fn get_bonding_curve_pda(mint: &Pubkey) -> Option<Pubkey> {
        let seeds: &[&[u8]; 2] = &[constants::seeds::BONDING_CURVE_SEED, mint.as_ref()];
        let program_id: &Pubkey = &cpi::ID;
        let pda: Option<(Pubkey, u8)> = Pubkey::try_find_program_address(seeds, program_id);
        pda.map(|pubkey| pubkey.0)
    }

    /// Gets the PDA for a token's metadata account
    pub fn get_metadata_pda(mint: &Pubkey) -> Pubkey {
        let seeds: &[&[u8]; 3] = &[
            constants::seeds::METADATA_SEED,
            constants::accounts::MPL_TOKEN_METADATA.as_ref(),
            mint.as_ref(),
        ];
        let program_id: &Pubkey = &constants::accounts::MPL_TOKEN_METADATA;
        Pubkey::find_program_address(seeds, program_id).0
    }

    /// Gets the global state account data
    pub fn get_global_account(&self) -> Result<accounts::GlobalAccount, error::ClientError> {
        let global: Pubkey = Self::get_global_pda();

        let account = self
            .rpc
            .get_account(&global)
            .map_err(error::ClientError::SolanaClientError)?;

        accounts::GlobalAccount::try_from_slice(&account.data)
            .map_err(error::ClientError::BorshError)
    }

    /// Gets a token's bonding curve account data
    pub fn get_bonding_curve_account(
        &self,
        mint: &Pubkey,
    ) -> Result<accounts::BondingCurveAccount, error::ClientError> {
        let bonding_curve_pda =
            Self::get_bonding_curve_pda(mint).ok_or(error::ClientError::BondingCurveNotFound)?;

        let account = self
            .rpc
            .get_account(&bonding_curve_pda)
            .map_err(error::ClientError::SolanaClientError)?;

        accounts::BondingCurveAccount::try_from_slice(&account.data)
            .map_err(error::ClientError::BorshError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_client::solana_sdk::signer::keypair::Keypair;

    #[test]
    fn test_new_client() {
        let payer = Keypair::new();
        let client = PumpFun::new(Cluster::Devnet, &payer, None, None);
        assert_eq!(client.payer.pubkey(), payer.pubkey());
    }

    #[test]
    fn test_get_pdas() {
        let mint = Keypair::new();
        let global_pda = PumpFun::get_global_pda();
        let mint_authority_pda = PumpFun::get_mint_authority_pda();
        let bonding_curve_pda = PumpFun::get_bonding_curve_pda(&mint.pubkey());
        let metadata_pda = PumpFun::get_metadata_pda(&mint.pubkey());

        assert!(global_pda != Pubkey::default());
        assert!(mint_authority_pda != Pubkey::default());
        assert!(bonding_curve_pda.is_some());
        assert!(metadata_pda != Pubkey::default());
    }
}
