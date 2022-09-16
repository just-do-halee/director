mod stages;

#[director::main(std::sync)]
fn main() {
    for _ in 0..10000 {
        // can be cycling
        stages::StateFoo::run(&mut Engine);
    }
}

pub struct Engine;
