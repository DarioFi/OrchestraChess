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

    pub(crate) fn max_allocable(&self) -> Duration {
        if self.move_time != 0 {
            return Duration::from_millis(self.move_time);
        } else {
            return Duration::from_millis(self.msec_left / 20 + (self.msec_inc as i32 * 0.8 as i32) as u64);
        }
    }
}

pub fn start_timer_maximum_allocable(millis: u128, hook: Arc<Mutex<bool>>){
    thread::spawn(move || {
        // Sleep for x milliseconds
        println!("Sleeping for {} seconds", millis / 1000);

        let seconds = millis / 1000;
        let millis_remaining = millis % 1000;

        for _ in 0..seconds {
            if *hook.lock().unwrap() {
                return;
            }
            thread::sleep(Duration::from_secs(1));
        }
        thread::sleep(Duration::from_millis(millis_remaining as u64));

        *hook.lock().unwrap() = true;
    });
}