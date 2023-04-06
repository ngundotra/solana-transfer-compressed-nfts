use anchor_lang::prelude::*;
use mpl_bubblegum::MintToCollectionV1;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod transfer_compressed {
    use mpl_bubblegum::cpi::accounts::MintToCollectionV1;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn mint(ctx: Context<Mint>) -> Result<()> {
        let accounts = MintToCollectionV1 {
            mint: ctx.accounts.mint_recipient.clone(),
            authority: ctx.accounts.authority.clone(),
            tree: ctx.accounts.tree.clone(),
            tree_authority: ctx.accounts.tree_authority.clone(),
            payer: ctx.accounts.payer.clone(),
        };
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>) -> Result<()> {
        Ok(())
    }

    pub fn mint_and_transfer(ctx: Context<MintAndTransfer>) -> Result<()> {
        // You can mint & transfer in this function without a proof because it was just minted
        // so the proof can be reconstructed from the on-chain buffer (aka a sort of cache)

        Ok(())
    }
}

pub const AUTHORITY_PREFIX: &str = "authority";

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    pub tree: AccountInfo<'info>,
    #[account(
        seeds=[AUTHORITY_PREFIX.as_bytes()],
        bump
    )]
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    /// CHECK:
    pub to: AccountInfo<'info>,
    /// CHECK:
    pub from: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    pub tree: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MintAndTransfer<'info> {
    /// CHECK:
    pub mint_recipient: AccountInfo<'info>,
    /// CHECK:
    pub to: AccountInfo<'info>,
    /// CHECK:
    pub from: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    pub tree: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Mint<'info> {
    /// CHECK:
    pub mint_recipient: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    /// CHECK:
    pub tree: AccountInfo<'info>,
}
