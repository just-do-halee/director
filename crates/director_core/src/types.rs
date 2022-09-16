pub trait State<Engine> {
    /// This determines whether or not to run this local state machine.
    fn toggle(engine: &mut Engine, inner: Option<&Self>) -> bool;
    /// This creates and imports new initial state on this local state machine when the toggle's on.
    fn load(engine: &mut Engine) -> Self;
    /// This executes custom logics and manipulates this local state machine's states.
    fn run(&mut self, engine: &mut Engine);
    /// When the toggle's off
    /// ...
    /// After then, the sub states[i.e) StateBar and StateBar2] will be droped automatically.
    fn drop(&mut self, engine: &mut Engine);
}
