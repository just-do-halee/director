pub mod ___ {
    //! Prerequirement elements
    pub use paste::paste;
}

pub use types::State;

pub use director_macros::main; // for #[director::main]
pub use director_macros::state; // for #[director::state]

#[macro_use]
mod macros;

mod types;
