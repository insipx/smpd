#![feature(associated_consts)]
#![feature(concat_idents)]
#![feature(use_extern_macros)]

#[macro_use]
mod macros;

pub mod SPC_DSP;
mod sizes;
mod registers;
mod state;
mod config;

