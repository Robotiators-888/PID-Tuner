mod pidcontroller;
use pidcontroller::PIDController;

const TARGETPOS: f64 = 10.0;
const P: f64 = 3.0;
const I: f64 = 0.1;
const D: f64 = 0.005;

fn main() {
    let mut pidc = PIDController::new(P, I, D, None);
    for i in 0..20 {
        println!("{}", pidc.calculate(i as f64,TARGETPOS));
    }
}