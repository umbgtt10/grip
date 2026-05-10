use crate::types::{CalcInput, Operation};

pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return 0.0;
    }
    a / b
}

pub fn compute(input: &CalcInput) -> f64 {
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
    !a.is_nan() && !b.is_nan() && !a.is_infinite() && !b.is_infinite()
}

fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("division by zero")
    } else {
        Ok(a / b)
    }
}
