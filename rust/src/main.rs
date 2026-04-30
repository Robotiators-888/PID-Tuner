mod pid;

// Most beautiful use to ever exist
use std::{
    io::{self, BufWriter, Write},
    sync::{Arc, Mutex},
    thread,
};

use pid::{
    controller::PIDController,
    tuner::{PID, tunePID},
};

const TARGETPOS: f64 = 10.0;
const P: f64 = 10.0;
const I: f64 = 0.0;
const D: f64 = 0.00;

fn main() {
    // Tells us the amount of threads
    let _threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1_usize); // Super idiomatic _usize
    let _pid: Arc<Mutex<PID>> = Arc::new(Mutex::new(PID {
        P: P,
        I: I,
        D: D,
        attempts: 0,
    }));
    let stdout = io::stdout().lock();
    let mut writer = BufWriter::new(stdout);
    let mut pidc = PIDController::new(P, I, D, None);
    let mut current_pos: f64 = 0.0;
    let mut attempts: u64 = 0;
    for _i in 0..100 {
        let calculation = pidc.calculate(current_pos, TARGETPOS);
        let _ = writeln!(
            writer,
            "Calculation: {}\nPosition: {}",
            &calculation, &current_pos
        );
        current_pos += calculation / 10.0;
        if current_pos != TARGETPOS {
            attempts += 1;
        }
    }
    let _ = writer.flush();
    println!("Attempts: {}", attempts);
    println!(
        "Tuned PID: {:?}",
        tunePID(
            &PID {
                P: 0.0,
                I: 0.0,
                D: 0.0,
                attempts: 100
            },
            TARGETPOS,
            200,
            |pos, out| { pos + out / 10.0 }
        )
    );
}
