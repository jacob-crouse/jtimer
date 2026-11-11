use std::time::{Duration, Instant};

struct Timer {
    charge_number: String,
    duration: Instant,
}

impl Timer {
    fn new(charge_number: String, duration: Instant) -> Timer {
	Timer {
	    charge_number, // these lines use field init shorthand
	    duration,
	    // eventually could put start, stop time in here
	    // should be able to create N timers and store in some database
	}
    }
}

fn main() {
    println!("Hello, world!");

    // eventually we'll read this in from somewhere, but hardcoding for now
    let charge_number = "RD000.123.456".to_string();
    let current_time  = Instant::now(); // sec

    // for the selected charge number, start a new timer
    let t1 = Timer::new(charge_number, current_time);

    // main function loop
    loop {
	println!("Charge Number: {}, Current Duration: {}", t1.charge_number, t1.duration.elapsed().as_secs());
    }
}
