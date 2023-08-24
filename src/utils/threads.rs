pub use api::Spawn;
pub use lib::Thread;

mod api {
    use std::thread;

    pub trait Spawn {
        fn spawn<F, T>(f: F) -> thread::JoinHandle<T>
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
        fn spawn<F, T>(f: F) -> thread::JoinHandle<T>
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static,
        {
            thread::spawn(f)
        }
    }
}
