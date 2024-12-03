use anchor_lang::prelude::*;

declare_id!("DK9ZAmcy3qCW15mDxLvhhKMVoqTRHdHbGSYHHmMBQGjL");

pub mod structs;
pub use structs::*;

#[program]
pub mod veritas_v2 {
    use super::*;

    pub fn create_collection(ctx: Context<CreateCollection>,collection_name:String, symbol:String) -> Result<()> {
        ctx.accounts.create_collection(&ctx.bumps, collection_name, symbol)
    }
    
    pub fn mint_nft(ctx: Context<MintNFT>,nft_name:String, symbol:String) -> Result<()> {
        ctx.accounts.mint_nft(&ctx.bumps, nft_name, symbol)
    }

    pub fn verify_collection(ctx: Context<VerifyCollectionMint>) -> Result<()> {
        ctx.accounts.verify_collection(&ctx.bumps)
    }
}

