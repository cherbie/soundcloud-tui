pub use api::*;
pub use lib::*;

mod api {
    use std::thread;

    pub trait ThreadHandle<T> {
        fn join(self) -> thread::Result<T>;
        fn is_finished(&self) -> bool;
    }

    pub trait Spawn {
        fn spawn<F, T>(f: F) -> Box<dyn ThreadHandle<T>>
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static;
    }
}

mod lib {
    use super::api::{Spawn, ThreadHandle};
    use std::thread;

    pub struct Thread;

    impl Default for Thread {
        fn default() -> Self {
            Self {}
        }
    }

    impl Spawn for Thread {
        fn spawn<F, T>(f: F) -> Box<dyn ThreadHandle<T>>
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static,
        {
            Box::new(thread::spawn(f))
        }
    }

    impl<T> ThreadHandle<T> for std::thread::JoinHandle<T> {
        fn is_finished(&self) -> bool {
            self.is_finished()
        }

        fn join(self) -> thread::Result<T> {
            self.join()
        }
    }
}
