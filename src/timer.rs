use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Timer {
    pub move_time: u64,
}


impl Timer {
    pub fn new_timer() -> Timer {
        Timer {
            move_time: 1000,
        }
    }

    fn how_much_time(&self) -> Duration {
        return Duration::from_millis(self.move_time)
    }
}

pub fn start_timer(x: Timer, hook: Arc<Mutex<bool>>) {
    thread::spawn(move || {
        // Sleep for x milliseconds
        thread::sleep(x.how_much_time()); // Change the duration as needed
        // Set the mutex to true after the specified time
        *hook.lock().unwrap() = true;
    });
}