pub trait Fixture {
    fn setup(&mut self);
    fn before_each(&mut self);
    fn after_each(&mut self);
    fn cleanup(&mut self);
}
