#![cfg_attr(target_os="solana",feature(asm_experimental_arch, asm_const))]
#![allow(unused)]

pub mod set_return;
pub use set_return::*;

pub mod set_register;
pub use set_register::*;