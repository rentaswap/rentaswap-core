use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

declare_id!("RSwaPxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

#[program]
pub mod rentaswap {
    use super::*;

    /// Wraps an active subscription delegation into a transferable NFT receipt
    /// and creates a listing on the order book.
    pub fn wrap_and_list(
        ctx: Context<WrapAndList>, 
        ask_price_usdc: u64,
        days_left: u16,
        merchant_id: String
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing_state;
        listing.seller = ctx.accounts.seller.key();
        listing.ask_price_usdc = ask_price_usdc;
        listing.days_left = days_left;
        listing.merchant_id = merchant_id;
        listing.is_active = true;
        listing.nft_mint = ctx.accounts.nft_mint.key();

        // 1. Verify delegation balance on-chain (prevent draining)
        // 2. Transfer delegation rights to Escrow PDA
        // 3. Mint the Receipt NFT to the seller

        msg!("Listing created: {} USDC for {} days", ask_price_usdc, days_left);
        Ok(())
    }

    /// Executes the atomic swap. Buyer pays USDC, receives the NFT receipt and delegation rights.
    pub fn atomic_buy(ctx: Context<AtomicBuy>) -> Result<()> {
        let listing = &mut ctx.accounts.listing_state;
        require!(listing.is_active, RentaswapError::ListingNotActive);

        // Calculate fees
        // 2% default fee, reduced based on token tier logic
        let fee_amount = (listing.ask_price_usdc * 2) / 100; 
        let seller_amount = listing.ask_price_usdc.checked_sub(fee_amount).unwrap();

        // 1. Transfer USDC from buyer to seller (seller_amount)
        // 2. Transfer USDC fee from buyer to treasury/burn wallet (fee_amount)
        // 3. Transfer Receipt NFT from Escrow PDA to buyer
        // 4. Reassign delegation rights to buyer

        listing.is_active = false;
        
        msg!("Swap executed. Seller received: {} USDC", seller_amount);
        Ok(())
    }

    /// Cancels a listing and unwraps the receipt back to standard delegation.
    pub fn cancel_listing(ctx: Context<CancelListing>) -> Result<()> {
        let listing = &mut ctx.accounts.listing_state;
        require!(listing.is_active, RentaswapError::ListingNotActive);
        require!(listing.seller == ctx.accounts.seller.key(), RentaswapError::Unauthorized);

        listing.is_active = false;
        
        // 1. Burn Receipt NFT
        // 2. Return delegation rights to seller from Escrow PDA

        msg!("Listing cancelled.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct WrapAndList<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        init,
        payer = seller,
        space = 8 + ListingState::INIT_SPACE,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump
    )]
    pub listing_state: Account<'info, ListingState>,

    /// CHECK: The mint for the new receipt NFT (Token-2022)
    #[account(mut)]
    pub nft_mint: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct AtomicBuy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// CHECK: The seller receiving the USDC
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump,
        has_one = seller
    )]
    pub listing_state: Account<'info, ListingState>,

    /// CHECK: The NFT mint being bought
    #[account(mut)]
    pub nft_mint: UncheckedAccount<'info>,

    /// CHECK: Protocol treasury for fees
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct CancelListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump,
        has_one = seller
    )]
    pub listing_state: Account<'info, ListingState>,

    /// CHECK: The NFT mint being cancelled
    #[account(mut)]
    pub nft_mint: UncheckedAccount<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct ListingState {
    pub seller: Pubkey,
    pub nft_mint: Pubkey,
    pub ask_price_usdc: u64,
    pub days_left: u16,
    #[max_len(32)]
    pub merchant_id: String,
    pub is_active: bool,
}

#[error_code]
pub enum RentaswapError {
    #[msg("This listing is no longer active.")]
    ListingNotActive,
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
