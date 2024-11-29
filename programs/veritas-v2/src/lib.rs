use anchor_lang::prelude::*;

declare_id!("Eapv22TSnRb3FLMAK4A886seiCgvs7NNoWd82A8sgR3w");

pub mod structs;
pub use structs::*;

#[program]
pub mod veritas_v2 {
    use super::*;
    
    pub fn create_collection(ctx: Context<CreateCollection>) -> Result<()> {
        ctx.accounts.create_collection(&ctx.bumps)
    }
    
    pub fn mint_nft(ctx: Context<MintNFT>) -> Result<()> {
        ctx.accounts.mint_nft(&ctx.bumps)
    }

    pub fn verify_collection(ctx: Context<VerifyCollectionMint>) -> Result<()> {
        ctx.accounts.verify_collection(&ctx.bumps)
    }
}

