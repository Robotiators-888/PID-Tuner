use std::time::Duration;

// Default would be nice but would give a period of 0
// #[Derive(Default)]
pub struct pidController {
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

impl pidController {
  pub fn new(Kp:f64, Ki:f64, Kd:f64, period: Option<u64>) -> pidController {
    match period {
      Some(v) => pidController{Kp:Kp,Ki:Ki,Kd:Kd,period:Duration::from_secs(v),prevError:0.0,error:0.0,totalError:0.0,minimumIntegral:1.0,maximumIntegral:-1.0},
      None => pidController{Kp:Kp,Ki:Ki,Kd:Kd,period:Duration::from_secs(20),prevError:0.0,error:0.0,totalError:0.0,minimumIntegral:1.0,maximumIntegral:-1.0}
    }
  }
  pub fn calculate(&mut self, measurement:f64, setpoint:f64) -> f64 {
    self.prevError = self.error;
    self.error = setpoint - measurement;
    // errorDerivative is part of the class in wpilib and not a local variable but there seems to be no reason for that
    let errorDerivative = (self.error - self.prevError) / self.period.as_secs() as f64;
    // Idk how this would be possible
    if self.error.abs() > f64::INFINITY {
      self.totalError = 0.0;
    }
    else if self.Ki != 0.0 {
      let tempval = self.totalError + self.error * self.period.as_secs() as f64;
      self.totalError = tempval.clamp(self.minimumIntegral / self.Ki, self.maximumIntegral / self.Ki)
    }
    self.Kp * self.error + self.Ki * self.totalError + self.Kd * errorDerivative
  }
}
