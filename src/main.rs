use std::time::{Duration, Instant};
use std::io::{self, Write};

struct Timer {
    charge_number: String, // charge number we're using for the timer
    duration: Instant, // instance of a counting up timer, used to count how long we've been on this charge number
    update_rate: f64, // Hz, how many times a second we'd like to update the displayed time
}

impl Timer {
    fn new(charge_number: String, duration: Instant, update_rate: f64) -> Timer {
	Timer {
	    charge_number, // these lines use field init shorthand
	    duration,
	    update_rate,
	    // eventually could put start, stop time in here
	    // should be able to create N timers and store in some database
	}
    }
}

fn update_screen(display_timer: &Timer, time_since_last_update: Instant) -> Instant {
    // given the update rate and the current time, write these values to stdout
    let total_time_elapsed = display_timer.duration.elapsed().as_secs() as f64; // this eventually this needs to be something better than as_secs() since that's just whole number seconds
    let update_period      = (1.0 / display_timer.update_rate).floor() as f64;
    let time_since_update  = time_since_last_update.elapsed().as_secs() as f64;
    let still_waiting;

    if time_since_update / update_period >= 1.0 { // time to update the screen!
	print!("Charge Number: {}, Current Duration: {}\r", display_timer.charge_number, total_time_elapsed);
	io::stdout().flush().unwrap();
	still_waiting = Instant::now()
    } else {
	still_waiting = time_since_last_update
    }

    still_waiting
}

fn main() {
    // eventually we'll read this in from somewhere, but hardcoding for now
    let charge_number = "RD000.123.456".to_string();
    let current_time  = Instant::now(); // sec
    let update_rate   = 1.0; // Hz

    // for the selected charge number, start a new timer
    let t1 = Timer::new(charge_number, current_time, update_rate);

    // main function loop
    let mut time_since_last_update = Instant::now();

    loop {
	time_since_last_update = update_screen(&t1, time_since_last_update);
    }
}
