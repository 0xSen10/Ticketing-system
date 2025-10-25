use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeMint, Mint, MintTo, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use mpl_token_metadata::instruction as token_metadata_instruction;
use mpl_token_metadata::state::Creator;
use solana_program::program::invoke_signed;

declare_id!("TICKET11111111111111111111111111111111111");

#[program]
pub mod ticket_platform {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        name: String,
        price_lamports: u64,
        sale_start: i64,
        sale_end: i64,
        total_tickets: u32,
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        require!(sale_start < sale_end, ErrorCode::InvalidTimeRange);
        event.organizer = ctx.accounts.organizer.key();
        event.name = name;
        event.price_lamports = price_lamports;
        event.sale_start = sale_start;
        event.sale_end = sale_end;
        event.total_tickets = total_tickets;
        event.sold_tickets = 0;
        event.bump = *ctx.bumps.get("vault").unwrap();
        Ok(())
    }

    pub fn organizer_mint_template(
        ctx: Context<OrganizerMintTemplate>,
        uri: String,
        title: String,
        symbol: String,
    ) -> Result<()> {
        let cpi_accounts = InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::initialize_mint(cpi_ctx, 0, &ctx.accounts.organizer.key(), Some(&ctx.accounts.organizer.key()))?;

        let metadata_accounts = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.organizer.to_account_info(),
            ctx.accounts.organizer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let creators = Some(vec![Creator {
            address: ctx.accounts.organizer.key(),
            verified: false,
            share: 100,
        }]);

        let ix = token_metadata_instruction::create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.organizer.key(),
            ctx.accounts.organizer.key(),
            ctx.accounts.organizer.key(),
            title,
            symbol,
            uri,
            creators,
            0,
            true,
            false,
            None,
            None,
        );

        invoke_signed(&ix, &metadata_accounts, &[])?;
        Ok(())
    }

    pub fn buy_ticket(
        ctx: Context<BuyTicket>,
        uri: String,
        title: String,
        symbol: String,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let event = &mut ctx.accounts.event;
        require!(now >= event.sale_start, ErrorCode::SaleNotStarted);
        require!(now <= event.sale_end, ErrorCode::SaleEnded);
        require!(event.sold_tickets < event.total_tickets, ErrorCode::SoldOut);

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.buyer.key(),
            &ctx.accounts.vault.key(),
            event.price_lamports,
        );
        invoke_signed(
            &ix,
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        let cpi_accounts = InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::initialize_mint(cpi_ctx, 0, &ctx.accounts.program_authority.key(), Some(&ctx.accounts.program_authority.key()))?;

        let cpi_accounts_mint_to = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.buyer_ata.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let cpi_ctx_mint = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts_mint_to);
        token::mint_to(cpi_ctx_mint.with_signer(&[&[b"authority", &[ctx.accounts.event.bump]]]), 1)?;

        let creators = Some(vec![Creator {
            address: ctx.accounts.organizer.key(),
            verified: false,
            share: 100,
        }]);

        let ix = token_metadata_instruction::create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.program_authority.key(),
            ctx.accounts.program_authority.key(),
            ctx.accounts.organizer.key(),
            title,
            symbol,
            uri,
            creators,
            0,
            true,
            false,
            None,
            None,
        );

        let accounts_for_metadata = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.program_authority.to_account_info(),
            ctx.accounts.organizer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        invoke_signed(&ix, &accounts_for_metadata, &[&[b"authority", &[ctx.accounts.event.bump]]])?;

        let mut ticket_info = TicketInfo {
            buyer: ctx.accounts.buyer.key(),
            mint: ctx.accounts.mint.key(),
            is_sold: true,
        };
        event.tickets.push(ticket_info);
        event.sold_tickets = event.sold_tickets.checked_add(1).ok_or(ErrorCode::ArithmeticOverflow)?;

        Ok(())
    }

    pub fn refund_ticket(ctx: Context<RefundTicket>) -> Result<()> {
        let event = &mut ctx.accounts.event;

        let mut found = false;
        for ticket in event.tickets.iter_mut() {
            if ticket.mint == ctx.accounts.mint.key() && ticket.buyer == ctx.accounts.buyer.key() {
                require!(ticket.is_sold, ErrorCode::InvalidTicketOwnership);
                ticket.is_sold = false;
                found = true;
                break;
            }
        }
        require!(found, ErrorCode::InvalidTicketOwnership);

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.vault.key(),
            &ctx.accounts.buyer.key(),
            event.price_lamports,
        );
        let vault_bump = event.bump;
        let seeds = &[b"vault", ctx.accounts.event.key().as_ref(), &[vault_bump]];
        invoke_signed(
            &ix,
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&seeds],
        )?;

        event.sold_tickets = event.sold_tickets.checked_sub(1).ok_or(ErrorCode::ArithmeticOverflow)?;
        Ok(())
    }

    pub fn withdraw_funds(ctx: Context<WithdrawFunds>) -> Result<()> {
        let event = &ctx.accounts.event;
        let now = Clock::get()?.unix_timestamp;
        require!(now > event.sale_end, ErrorCode::SaleNotEnded);

        let vault_bump = event.bump;
        let seeds = &[b"vault", ctx.accounts.event.key().as_ref(), &[vault_bump]];
        let balance = ctx.accounts.vault.to_account_info().lamports();
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.vault.key(),
            &ctx.accounts.organizer.key(),
            balance,
        );
        invoke_signed(
            &ix,
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.organizer.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&seeds],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String, price_lamports: u64, sale_start: i64, sale_end: i64, total_tickets: u32)]
pub struct CreateEvent<'info> {
    #[account(init, payer = organizer, space = 8 + Event::MAX_SIZE)]
    pub event: Account<'info, Event>,
    #[account(
        init,
        payer = organizer,
        seeds = [b"vault", event.key().as_ref()],
        bump,
        space = 8,
    )]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OrganizerMintTemplate<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,
    #[account(init, payer = organizer, mint::decimals = 0, mint::authority = organizer)]
    pub mint: Account<'info, Mint>,
    pub metadata: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut, has_one = organizer)]
    pub event: Account<'info, Event>,
    #[account(mut, seeds = [b"vault", event.key().as_ref()], bump = event.bump)]
    pub vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(init, payer = buyer, mint::decimals = 0, mint::authority = program_authority, seeds = [b"mint", event.key().as_ref(), &[event.sold_tickets as u8]], bump)]
    pub mint: Account<'info, Mint>,

    #[account(mut, seeds = [b"authority"], bump = event.bump)]
    pub program_authority: UncheckedAccount<'info>,

    #[account(init_if_needed, payer = buyer, associated_token::mint = mint, associated_token::authority = buyer)]
    pub buyer_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    pub organizer: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RefundTicket<'info> {
    #[account(mut, has_one = organizer)]
    pub event: Account<'info, Event>,
    #[account(mut, seeds = [b"vault", event.key().as_ref()], bump = event.bump)]
    pub vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut, has_one = organizer)]
    pub event: Account<'info, Event>,
    #[account(mut, seeds = [b"vault", event.key().as_ref()], bump = event.bump)]
    pub vault: UncheckedAccount<'info>,
    #[account(mut, address = event.organizer)]
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Event {
    pub organizer: Pubkey,
    pub name: String,
    pub price_lamports: u64,
    pub sale_start: i64,
    pub sale_end: i64,
    pub total_tickets: u32,
    pub sold_tickets: u32,
    pub bump: u8,
    pub tickets: Vec<TicketInfo>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TicketInfo {
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub is_sold: bool,
}

impl Event {
    pub const MAX_SIZE: usize = 32 + 4 + 64 + 8 + 8 + 8 + 4 + 4 + 1 + (4 + 100 * (32 + 32 + 1));
}

#[error_code]
pub enum ErrorCode {
    #[msg("Sale not started yet.")]
    SaleNotStarted,
    #[msg("Sale has already ended.")]
    SaleEnded,
    #[msg("Tickets sold out.")]
    SoldOut,
    #[msg("Invalid time range for sale.")]
    InvalidTimeRange,
    #[msg("Buyer does not own the ticket NFT.")]
    InvalidTicketOwnership,
    #[msg("Sale period has not ended.")]
    SaleNotEnded,
    #[msg("Arithmetic overflow.")]
    ArithmeticOverflow,
}
