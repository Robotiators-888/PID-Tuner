use std::time::Duration;

// Default would be nice but would give a period of 0
// #[Derive(Default)]
pub struct PIDController {
  kp: f64,
  ki: f64,
  kd: f64,
  period: Duration,
  prev_error: f64,
  error: f64,
  total_error: f64,
  minimum_integral: f64,
  maximum_integral: f64
}

impl PIDController {
  pub fn new(kp:f64, ki:f64, kd:f64, period: Option<u64>) -> PIDController {
    match period {
      Some(v) => PIDController{kp,ki,kd,period:Duration::from_secs(v),prev_error:0.0,error:0.0,total_error:0.0,minimum_integral:-1.0,maximum_integral:1.0},
      None => PIDController{kp,ki,kd,period:Duration::from_secs(20),prev_error:0.0,error:0.0,total_error:0.0,minimum_integral:-1.0,maximum_integral:1.0}
    }
  }
  pub fn calculate(&mut self, measurement:f64, setpoint:f64) -> f64 {
    self.prev_error = self.error;
    self.error = setpoint - measurement;
    // errorDerivative is part of the class in wpilib and not a local variable but there seems to be no reason for that
    let error_derivative = (self.error - self.prev_error) / self.period.as_secs() as f64;
    // Idk how this would be possible
    if self.error.abs() > f64::INFINITY {
      self.total_error = 0.0;
    }
    else if self.ki != 0.0 {
      let tempval = self.total_error + self.error * self.period.as_secs() as f64;
      self.total_error = tempval.clamp(self.minimum_integral / self.ki, self.maximum_integral / self.ki)
    }
    self.kp * self.error + self.ki * self.total_error + self.kd * error_derivative
  }
}
