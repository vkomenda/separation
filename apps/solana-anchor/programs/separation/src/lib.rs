use anchor_lang::prelude::*;
// TODO: find out how to include `core` into the build.
//use core::{say_hello_world, HelloWorldArray};
use solana_program::program_error::ProgramError;

// Temporary replacement for the `core` crate.
pub mod core {
    use std::borrow::BorrowMut;

    pub type HelloWorldArray = [u8; 11];

    #[derive(Debug)]
    pub struct HelloWorldStruct<'a> {
        pub info: u64,
        /// This field simulates a complex subaccount dependency and is supposed to be stored in a
        /// separate Solana account.
        pub array: &'a mut HelloWorldArray,
    }

    /// Mutates the `array` of a `HelloWorldStruct`.
    pub fn say_hello_world(hws: &mut HelloWorldStruct) {
        let hw = b"hello world";
        let slice: &mut [u8] = hws.array.borrow_mut();
        slice.copy_from_slice(&hw[..]);
    }
}

declare_id!("JCAz2DvWGDmxrBGA182VPipYNXLyB5Z6vybi6A1pZcmf");

#[program]
pub mod separation {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let container = &ctx.accounts.container;
        let hw: &mut Account<HelloWorld> = &mut ctx.accounts.hello_world;

        // Consistency theck for the subaccount field. This should be automated somehow.
        if container.hello_world_subaccount != hw.key() {
            return Err(ProgramError::InvalidAccountData);
        }

        // TODO: This construction of a core struct should be made into an adapter.
        let mut hws: core::HelloWorldStruct = core::HelloWorldStruct {
            info: container.info,
            array: &mut hw.array,
        };

        // The application calls the core method. Instead of creating `hws` above, it is possible to
        // use an adaptor method that takes `container` and `hw` and forwards those to the core
        // method.
        core::say_hello_world(&mut hws);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub container: Account<'info, HelloWorldContainer>,
    /// Since `container` isn't enough to obtain the `AccountInfo` of it's `hello_world_subaccount`,
    /// that `AccountInfo` is provided to the instruction directly.
    #[account(mut)]
    pub hello_world: Account<'info, HelloWorld>,
}

#[account]
pub struct HelloWorld {
    /// This field is supposed to be accessed from a `HelloWorldContainer`.
    pub array: core::HelloWorldArray,
}

/// This container account corresponds to `core::HelloWorldStruct`. However the reference to the
/// array is made using a subaccount pubkey.
#[account]
pub struct HelloWorldContainer {
    /// Some information about the container.
    pub info: u64,
    /// A link to the subaccount of type `HelloWorld`.
    pub hello_world_subaccount: Pubkey,
}
