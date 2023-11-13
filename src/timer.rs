use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Timer {
    pub move_time: u64,
    pub msec_left: u64,
    pub msec_inc: u64,
}


impl Timer {
    pub fn new_timer() -> Timer {
        Timer {
            move_time: 0,
            msec_left: 0,
            msec_inc: 0,
        }
    }

    fn how_much_time(&self) -> Duration {
        if self.move_time != 0 {
            return Duration::from_millis(self.move_time);
        } else {
            return Duration::from_millis(self.msec_left / 10 + (self.msec_inc as i32 * 0.8 as i32) as u64);
        }
    }
}

pub fn start_timer(x: Timer, hook: Arc<Mutex<bool>>) {
    thread::spawn(move || {
        // Sleep for x milliseconds
        println!("Sleeping for {} seconds", x.how_much_time().as_millis() as i32 / 1000 as i32);
        thread::sleep(x.how_much_time()); // Change the duration as needed
        // Set the mutex to true after the specified time
        *hook.lock().unwrap() = true;
    });
}