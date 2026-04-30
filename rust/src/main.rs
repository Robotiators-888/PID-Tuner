mod pidcontroller;

// Most beautiful use to ever exist
use std::{thread, sync::{Arc, Mutex}, io::{self, BufWriter, Write}};

use pidcontroller::PIDController;

const TARGETPOS: f64 = 10.0;
const P: f64 = 10.0;
const I: f64 = 0.0;
const D: f64 = 0.00;

fn main() {
    // Tells us the amount of threads
    let _threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1_usize); // Super idiomatic _usize
    let _pid: Arc<Mutex<PID>> = Arc::new(Mutex::new(PID{P:P,I:I,D:D,attempts:0}));
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
    println!("Tuned PID: {:?}", tunePID(&PID{P:P,I:I,D:D,attempts}, TARGETPOS, 200));
    println!("Meta Tuned PID: {:?}", metaTunePID(TARGETPOS, 200));
}

// I might actually need to use a PID to tune the PID
#[allow(non_snake_case)]
fn metaTunePID(target: f64, attempts: u64) -> PID {
    let mut current_PID: PID = PID{P:0.0,I:0.0,D:0.0,attempts:100};
    let mut last_attempt = current_PID.attempts;
    let mut P_tune_val = 1.0;
    let mut I_tune_val = 0.05;
    let mut _D_tune_val = 0.0;
    for _ in 0..attempts {
        current_PID.P += P_tune_val;
        let result = tunePID(&current_PID, target, 200);
        if result.attempts > last_attempt {
            current_PID.P -= P_tune_val;
            P_tune_val /= 10.0;
        }
        current_PID.I += I_tune_val;
        let result = tunePID(&current_PID, target, 200);
        if result.attempts > last_attempt {
            current_PID.I -= I_tune_val;
            I_tune_val /= 10.0;
        }
        let result = tunePID(&current_PID, target, 100);
        last_attempt = result.attempts;
        // Just ignore D for now
    }
    current_PID.attempts = last_attempt;
    current_PID
}

// I might actually need to use a PID to tune the PID
#[allow(non_snake_case)]
fn tunePID(best_PID: &PID, target: f64, attempts: u64) -> PID {
    let mut current_PID: PID = PID::clone(best_PID);
    let mut last_attempt = best_PID.attempts;
    let mut P_tune_val = 1.0;
    let mut I_tune_val = 0.05;
    let mut _D_tune_val = 0.0;
    for _ in 0..attempts {
        current_PID.P += P_tune_val;
        let result = simulate_attempts(&current_PID, target, |pos, calc| {pos+calc/10.0}, 100);
        if result > last_attempt {
            current_PID.P -= P_tune_val;
            P_tune_val /= 10.0;
        }
        current_PID.I += I_tune_val;
        let result = simulate_attempts(&current_PID, target, |pos, calc| {pos+calc/10.0}, 100);
        if result > last_attempt {
            current_PID.I -= I_tune_val;
            I_tune_val /= 10.0;
        }
        let result = simulate_attempts(&current_PID, target, |pos, calc| {pos+calc/10.0}, 100);
        last_attempt = result;
        // Just ignore D for now
    }
    current_PID.attempts = last_attempt;
    current_PID
}

fn simulate_attempts (pid: &PID, target: f64, simfunc: fn (pos: f64, pid_output: f64) -> f64, repetitions: u64) -> u64 {
    let mut pidc = PIDController::new(pid.P, pid.I, pid.D, None);
    let mut attempts: u64 = 0;
    let mut pos: f64 = 0.0;
    for _ in 0..repetitions {
        pos = simfunc(pos, pidc.calculate(pos, target));
        if pos != target {
            attempts += 1;
        }
    }
    attempts
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PID {
    pub P: f64,
    pub I: f64,
    pub D: f64,
    pub attempts: u64
}