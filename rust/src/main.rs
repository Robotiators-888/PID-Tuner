mod pidcontroller;

use std::io::{self, BufWriter, Write};

use pidcontroller::PIDController;

const TARGETPOS: f64 = 10.0;
const P: f64 = 3.0;
const I: f64 = 0.1;
const D: f64 = 0.005;

fn main() {
    let stdout = io::stdout().lock();
    let mut writer = BufWriter::new(stdout);
    let mut pidc = PIDController::new(P, I, D, None);
    let mut current_pos: f64 = 0.0;
    for _i in 0..20 {
        let calculation = pidc.calculate(current_pos, TARGETPOS);
        let _ = writeln!(writer, "Calculation: {}\nPosition: {}", &calculation, &current_pos);
        current_pos += calculation/10.0;
    }
    let _ = writer.flush();
}
