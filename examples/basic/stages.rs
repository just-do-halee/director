#![allow(unused_variables, dead_code)]

use crate::Engine;

#[director::state {
    sub = StateSub,
}]
#[derive(Debug)]
pub struct StateFoo {
    count: i64,
}

impl director::State<Engine> for StateFoo {
    fn toggle(engine: &mut Engine, inner: Option<&Self>) -> bool {
        director::on!(inner, None)
            || director::off!(state: StateSub, Some(state) if state.sub_count > 200)
    }
    fn load(engine: &mut Engine) -> Self {
        Self { count: 0 }
    }

    fn run(&mut self, engine: &mut Engine) {
        self.count -= 1;
        println!("super: {}", self.count);
    }

    fn drop(&mut self, engine: &mut Engine) {
        println!("{}", self.count);
    }
}

#[director::state {
    super = StateFoo
}]
#[derive(Debug)]
pub struct StateSub {
    sub_count: u64,
}

impl director::State<Engine> for StateSub {
    fn toggle(engine: &mut Engine, inner: Option<&Self>) -> bool {
        Self::lock_super__state_foo().get().count < -100
    }
    fn load(engine: &mut Engine) -> Self {
        Self { sub_count: 0 }
    }

    fn run(&mut self, engine: &mut Engine) {
        self.sub_count += 1;
        println!("sub: {}", self.sub_count);
    }

    fn drop(&mut self, engine: &mut Engine) {
        println!("{}", self.sub_count);
    }
}
