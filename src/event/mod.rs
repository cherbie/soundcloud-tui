mod events;
mod input_reader;
mod key;

#[cfg(test)]
mod tests;

pub use self::{
    events::{Event, Events},
    key::Key,
};
