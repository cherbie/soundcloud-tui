pub use api::*;
pub use lib::*;

mod api {
    use super::super::utils::{EventPoll, EventRead};
    use std::time::Duration;

    #[derive(Debug, Clone, Copy)]
    pub struct EventConfig {
        pub tick_rate: Duration,
    }

    pub trait EventServer {
        type Event;

        fn config(&self) -> EventConfig;
        fn listen<S>(&mut self, event_source: S)
        where
            S: EventPoll + EventRead<Event = Self::Event> + Send + 'static;
        fn stop(&mut self);
    }
}

mod lib {
    use super::api::{EventConfig, EventServer};
    use crate::event::{
        utils::{poll_for_event, EventPoll, EventRead},
        Event,
    };
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    ////// EventConfig ///////
    impl Default for EventConfig {
        fn default() -> EventConfig {
            EventConfig {
                tick_rate: Duration::from_millis(250),
            }
        }
    }

    ///////// EventServerCore ////////
    pub struct EventServerCore<T> {
        config: EventConfig,
        receiver: mpsc::Receiver<Event<T>>,
        sender: mpsc::Sender<Event<T>>,
        handle: Option<thread::JoinHandle<()>>,
        should_stop: Arc<Mutex<bool>>,
    }

    impl<T> Default for EventServerCore<T> {
        fn default() -> Self {
            let (sender, receiver) = mpsc::channel();

            EventServerCore {
                config: EventConfig::default(),
                receiver,
                sender,
                handle: None,
                should_stop: Arc::new(Mutex::new(false)),
            }
        }
    }

    impl<T> EventServer for EventServerCore<T>
    where
        T: Send + 'static,
    {
        type Event = T;

        fn config(&self) -> EventConfig {
            self.config
        }

        fn listen<S>(&mut self, event_source: S)
        where
            S: EventPoll + EventRead<Event = Self::Event> + Send + 'static,
        {
            let event_sender = self.sender.clone();
            let tick_rate = self.config.tick_rate;
            let should_stop = Arc::clone(&self.should_stop);

            let join_handle = thread::spawn(move || loop {
                {
                    if *should_stop.lock().unwrap() {
                        break;
                    }
                }
                poll_for_event(&event_source, &event_sender, tick_rate)
            });

            self.handle = Some(join_handle);
        }

        fn stop(&mut self) {
            self.stop()
        }
    }

    impl<T> EventServerCore<T> {
        fn stop(&mut self) {
            {
                let mut exit_condition = self.should_stop.lock().unwrap();
                *exit_condition = true;
            }

            if self.handle.is_some() {
                self.handle
                    .take()
                    .unwrap()
                    .join()
                    .expect("error joining thread handle");
            }
        }
    }

    impl<T> Drop for EventServerCore<T> {
        fn drop(&mut self) {
            self.stop();
        }
    }

    impl<T> Iterator for EventServerCore<T> {
        type Item = Event<T>;

        fn next(&mut self) -> Option<Self::Item> {
            self.receiver.recv().ok()
        }
    }
}
