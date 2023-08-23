#[cfg(test)]
pub use api::mock;
pub use api::Spawn;
pub use lib::Thread;

mod api {
    use std::thread;

    #[cfg(test)]
    use unimock::unimock;

    #[cfg_attr(test, unimock(api=mock))]
    pub trait Spawn {
        fn spawn<F, T>(&self, f: F) -> thread::JoinHandle<T>
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static;
    }
}

mod lib {
    use super::api::Spawn;
    use std::thread;

    pub struct Thread;

    impl Default for Thread {
        fn default() -> Self {
            Self {}
        }
    }

    impl Spawn for Thread {
        fn spawn<F, T>(&self, f: F) -> thread::JoinHandle<T>
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static,
        {
            thread::spawn(f)
        }
    }
}
