#![feature(associated_consts)]
#![feature(concat_idents)]

#[macro_use]
mod macros;

pub type sample_t = i16;
pub const NULL_U8:*mut u8 = 0 as *mut u8;
pub const NULL_SAMPLE_T:*mut sample_t = 0 as *mut sample_t;
pub mod SPC_DSP;
mod sizes;
mod registers;
mod state;
mod config;


