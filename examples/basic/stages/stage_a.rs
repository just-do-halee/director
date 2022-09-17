use super::stage_b::StateSub;
use crate::Engine;

#[director::state {
    sub = StateSub,
}]
#[derive(Debug)]
pub struct StateFoo {
    pub count: i64,
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
