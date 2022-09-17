use super::stage_a::StateFoo;
use crate::Engine;

#[director::state {
    super = StateFoo
}]
#[derive(Debug)]
pub struct StateSub {
    pub sub_count: u64,
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
