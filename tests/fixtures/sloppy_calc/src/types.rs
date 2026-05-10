use std::fmt;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn symbol(&self) -> &str {
        match self {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "*",
            Operation::Divide => "/",
        }
    }

    fn apply(&self, a: f64, b: f64) -> f64 {
        match self {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
        }
    }
}

struct CalcInput {
    a: f64,
    b: f64,
    op: Operation,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}
