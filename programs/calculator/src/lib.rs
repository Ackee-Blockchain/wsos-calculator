use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("2VsDgBxxmv6cyKAJjjmKHggt777rN4ELvQ2W6yCzzpqp");

#[program]
pub mod calculator {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok({})
    }

    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct Create<'info> {

    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    greeting: String,
    result: i64
}


