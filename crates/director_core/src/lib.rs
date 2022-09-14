pub mod ___ {
    //! Prerequirement elements
    pub use super::{
        lazy_static, paste,
        types::{StateGuard, StateOrigin},
        StateController,
    };
}

// external
pub use lazy_static::lazy_static;
pub use paste::paste;

pub use director_macros::attribute_state; // for #[director::state]

// internal
pub use types::State;

pub use state_controller::StateController;
mod state_controller {
    use crate::types::{State as StateTrait, *};
    use core::marker::PhantomData;

    pub struct StateController<'a, Engine, State: StateTrait<Engine>> {
        state_guard: StateGuard<'a, State>,
        phantomdata: PhantomData<Engine>,
    }

    impl<'a, Engine, State> StateController<'a, Engine, State>
    where
        State: StateTrait<Engine>,
    {
        #[inline]
        pub fn new(state_guard: StateGuard<'a, State>) -> Self {
            Self {
                state_guard,
                phantomdata: PhantomData,
            }
        }

        #[inline]
        pub fn into_inner(self) -> StateGuard<'a, State> {
            self.state_guard
        }
        #[inline]
        pub fn as_inner(&self) -> &StateGuard<'a, State> {
            &self.state_guard
        }
        #[inline]
        pub fn as_mut_inner(&mut self) -> &mut StateGuard<'a, State> {
            &mut self.state_guard
        }

        #[inline]
        pub fn get_option(&self) -> Option<&State> {
            self.as_inner().as_ref()
        }
        #[inline]
        pub fn get_mut_option(&mut self) -> Option<&mut State> {
            self.as_mut_inner().as_mut()
        }

        #[inline]
        pub fn get(&self) -> &State {
            self.as_inner().as_ref().unwrap()
        }
        #[inline]
        pub fn get_mut(&mut self) -> &mut State {
            self.as_mut_inner().as_mut().unwrap()
        }

        #[inline]
        pub fn set(&mut self, value: Option<State>) {
            **self.as_mut_inner() = value;
        }

        #[inline]
        pub fn unlock(self) {}
    }
}

mod types;

#[macro_use]
mod macros;
