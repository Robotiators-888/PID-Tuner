use super::controller::PIDController;

#[allow(non_snake_case)]
pub fn tunePID(
    best_PID: &PID,
    target: f64,
    attempts: u64,
    simfunc: fn(pos: f64, pid_output: f64) -> f64,
) -> PID {
    let mut current_PID: PID = PID::clone(best_PID);
    let mut last_attempt = best_PID.attempts;
    let mut P_tune_val = 1.0;
    let mut I_tune_val = 0.05;
    let mut D_tune_val = 0.001;
    for _ in 0..attempts {
        current_PID.P += P_tune_val;
        let result = simulate_attempts(&current_PID, target, simfunc, 100);
        if result > last_attempt {
            current_PID.P -= P_tune_val;
            P_tune_val /= 10.0;
        }
        let result = simulate_attempts(&current_PID, target, simfunc, 100);
        last_attempt = result;
        // Just ignore D for now
    }
    for _ in 0..attempts {
        current_PID.I += I_tune_val;
        let result = simulate_attempts(&current_PID, target, simfunc, 100);
        if result > last_attempt {
            current_PID.I -= I_tune_val;
            I_tune_val /= 10.0;
        }
        let result = simulate_attempts(&current_PID, target, simfunc, 100);
        last_attempt = result;
    }
    for _ in 0..attempts {
        current_PID.D += D_tune_val;
        let result = simulate_attempts(&current_PID, target, simfunc, 100);
        if result > last_attempt {
            current_PID.D -= D_tune_val;
            D_tune_val /= 10.0;
        }
        let result = simulate_attempts(&current_PID, target, simfunc, 100);
        last_attempt = result;
    }
    current_PID.attempts = last_attempt;
    current_PID
}

// Maybe make private
pub fn simulate_attempts(
    pid: &PID,
    target: f64,
    simfunc: fn(pos: f64, pid_output: f64) -> f64,
    repetitions: u64,
) -> u64 {
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
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PID {
    pub P: f64,
    pub I: f64,
    pub D: f64,
    pub attempts: u64,
}
