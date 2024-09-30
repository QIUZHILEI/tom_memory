#![no_std]
#![feature(error_generic_member_access)]
#[cfg(feature = "dma")]
mod dma;
mod err;
#[cfg(feature = "page")]
mod page;
#[cfg(feature = "phy")]
mod phy;
#[cfg(feature = "vm")]
mod vm;
#[cfg(feature = "dma")]
pub use dma::DmaAllocator;
pub use err::*;
#[cfg(feature = "page")]
pub use page::*;
#[cfg(feature = "phy")]
pub use phy::*;
#[cfg(feature = "vm")]
pub use vm::*;

