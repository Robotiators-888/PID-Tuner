mod pidcontroller;
use mathcore::MathCore;
use pidcontroller::PIDController;

const TARGETPOS: f64 = 10.0;
const P: f64 = 3.0;
const I: f64 = 0.1;
const D: f64 = 0.005;
const LASTVALDAMPENER: f64 = 0.7;

fn print_type<T>(_x: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let mut pidc = PIDController::new(P, I, D, None);
    let integral = MathCore::numerical_integrate("x^2", "x", 0.0, 1.0).unwrap();
    let other_integral = MathCore::integrate("2*x", "x").unwrap();
    print_type(&other_integral);
    print_type(&integral);
    println!("{}", other_integral);
    println!("{}", integral);
    let mut prev_val: f64 = 0.0;
    for i in 0..20 {
        println!("{}", pidc.calculate(i as f64,10.0));
        println!("PID: {}", pid(0, prev_val));
        println!("Error: {}", error(0, prev_val));
        prev_val = pid(0, prev_val);
    }
}

fn pid(x: i32, prev_val: f64) -> f64 {
    let math = MathCore::new();
    let pval = P * error(x, prev_val);
    let ival = I * MathCore::numerical_integrate(
        format!("{}", error(x, prev_val)).as_str(),
        "x",
        0.0,
        x as f64,
    )
    .unwrap();
    let dval = D * math
        .calculate(
            format!(
                "{}",
                MathCore::differentiate(format!("{}", error(x, prev_val)).as_str(), "x").unwrap()
            )
            .replace('x', format!("{}", x).as_str())
            .as_str(),
        )
        .unwrap();
    pval + ival + dval
}

fn error(_x: i32, prev_val: f64) -> f64 {
    let result = TARGETPOS - (prev_val * LASTVALDAMPENER);
    result
}
