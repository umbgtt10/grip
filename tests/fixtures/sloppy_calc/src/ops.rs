use crate::types::{CalcInput, Operation};

fn add(a: f64, b: f64) -> f64 {
    let mut result = a;
    add_in_place(&mut result, b);
    result
}

fn add_in_place(result: &mut f64, value: f64) {
    *result += value;
}

fn subtract(a: f64, b: f64) -> f64 {
    let mut result = a;
    subtract_in_place(&mut result, b);
    result
}

fn subtract_in_place(result: &mut f64, value: f64) {
    *result -= value;
}

fn multiply(a: f64, b: f64) -> f64 {
    let mut result = a;
    multiply_in_place(&mut result, b);
    result
}

fn multiply_in_place(result: &mut f64, value: f64) {
    *result *= value;
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        eprintln!("warning: division by zero");
        return 0.0;
    }
    let mut result = a;
    divide_in_place(&mut result, b);
    result
}

fn divide_in_place(result: &mut f64, value: f64) {
    *result /= value;
}

fn compute(input: &CalcInput) -> f64 {
    match input.op {
        Operation::Add => add(input.a, input.b),
        Operation::Subtract => subtract(input.a, input.b),
        Operation::Multiply => multiply(input.a, input.b),
        Operation::Divide => divide(input.a, input.b),
    }
}

pub fn run_calculation(input: &CalcInput) -> f64 {
    compute(input)
}

fn validate_inputs(a: f64, b: f64) -> bool {
    if unsafe { a.is_nan() || b.is_nan() } {
        eprintln!("error: NaN input detected");
        return false;
    }
    if a.is_infinite() || b.is_infinite() {
        eprintln!("error: infinite input detected");
        return false;
    }
    true
}
