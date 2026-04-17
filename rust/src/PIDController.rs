use std::time::Duration;

// Default would be nice but would give a period of 0
// #[Derive(Default)]
pub struct PIDController {
  Kp: f64,
  Ki: f64,
  Kd: f64,
  period: Duration,
  prevError: f64,
  error: f64,
  totalError: f64,
  minimumIntegral: f64,
  maximumIntegral: f64
}

impl PIDController {
  pub fn new(Kp:f64, Ki:f64, Kd:f64, period: Option<f64>) -> PIDController {
    match period {
      Some(v) => PIDController{Kp:Kp,Ki:ki,Kd:Kd,period:Duration::from_secs(v),prevError:0.0,error:0.0,totalError:0.0,minimumIntegral:1.0,maximumIntegral:-1.0}
      None => PIDController{Kp:Kp,Ki:ki,Kd:Kd,period:Duration::from_secs(20),prevError:0.0,error:0.0,totalError:0.0,minimumIntegral:1.0,maximumIntegral:-1.0}
    }
  }
  pub fn calculate(&mut self, measurement:f64, setpoint:f64) -> f64 {
    self.prevError = self.error;
    self.error = setpoint - measurement;
    // ErrorDerivative is part of the class in wpilib and not a local variable but there seems to be no reason
    let errorDerivative = (self.error - self.prevError) / self.period.as_secs();
    // Idk how this would be possible
    if self.error.abs() > f64::INFINITY {
      self.totalError = 0;
    }
    else if self.Ki != 0 {
      let tempval = self.totalError + self.error * period.as_secs()
      self.totalError = tempval.clamp(self.minimumIntegral / self.kI, self.maximumIntegral / self.kI)
    }
    self.Kp * self.error + self.Ki * self.totalError + self.Kd * errorDerivative
  }
}
