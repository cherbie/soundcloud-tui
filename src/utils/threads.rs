#[cfg(test)]
mod test {
    pub use mockall::predicate::*;
    pub use mockall::*;
}
use std::thread;

#[cfg(test)]
use test::*;

#[cfg_attr(test, test::automock)]
pub trait Spawn {
    fn spawn<F, T>(f: F) -> thread::JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static;
}

pub struct ThreadUtils;

impl Spawn for ThreadUtils {
    fn spawn<F, T>(f: F) -> thread::JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        thread::spawn(f)
    }
}
