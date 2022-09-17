#![no_std]
#![allow(unused_variables, dead_code)]

#[director::state{
    sub = StateA2
}]
#[derive(Debug)]
pub struct StateA {
    pub count: u8,
}
impl director::State<Engine> for StateA {
    fn toggle(engine: &mut Engine, inner: Option<&Self>) -> bool {
        director::on!(inner, None)
    }
    fn load(engine: &mut Engine) -> Self {
        todo!()
    }
    fn run(&mut self, engine: &mut Engine) {
        todo!()
    }
    fn drop(&mut self, engine: &mut Engine) {
        todo!()
    }
}

#[director::state {
    super = StateA
}]
#[derive(Debug)]
pub struct StateA2 {
    pub count: u8,
}
impl director::State<Engine> for StateA2 {
    fn toggle(engine: &mut Engine, inner: Option<&Self>) -> bool {
        todo!()
    }
    fn load(engine: &mut Engine) -> Self {
        todo!()
    }
    fn run(&mut self, engine: &mut Engine) {
        todo!()
    }
    fn drop(&mut self, engine: &mut Engine) {
        todo!()
    }
}
type Engine = ();

#[director::main(spin)]
fn main() {}
