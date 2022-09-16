#![cfg_attr(not(feature = "std"), no_std)]

pub mod ___ {
    //! Prerequirement elements
    pub use lazy_static::lazy_static;
    pub use paste::paste;
}

pub use director_macros::main; // for #[director::main]
pub use director_macros::state; // for #[director::state]

#[macro_use]
mod macros;

mod types;

pub use types::State;
