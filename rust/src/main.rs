use mathcore::MathCore;

const startingError: i32 = 10;
const P: f64 = 3.8;
const I: f64 = 0.7;
const D: f64 = 0.3;
const lastValDampener: f64 = -0.3;

fn print_type<T>(_x: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let integral = MathCore::numerical_integrate("x^2", "x", 0.0, 1.0).unwrap();
    let otherIntegral = MathCore::integrate("2*x", "x").unwrap();
    print_type(&otherIntegral);
    print_type(&integral);
    println!("{}", otherIntegral);
    println!("{}", integral);
    let mut prevVal: f64 = 0.0;
    for i in 0..20 {
        println!("PID: {}", PID(0, prevVal));
        println!("Error: {}", Error(0, prevVal));
        prevVal = PID(0, prevVal);
    }
}

fn PID(x: i32, prevVal: f64) -> f64 {
    let math = MathCore::new();
    let pval = P * Error(x, prevVal);
    let ival = I * MathCore::numerical_integrate(
        format!("{}", Error(x, prevVal)).as_str(),
        "x",
        0.0,
        x as f64,
    )
    .unwrap();
    let dval = D * math
        .calculate(
            format!(
                "{}",
                MathCore::differentiate(format!("{}", Error(x, prevVal)).as_str(), "x").unwrap()
            )
            .replace('x', format!("{}", x).as_str())
            .as_str(),
        )
        .unwrap();
    pval + ival + dval
}

fn Error(x: i32, prevVal: f64) -> f64 {
    (prevVal * lastValDampener) + startingError as f64
}
