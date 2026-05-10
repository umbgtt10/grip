use crate::ops::run_calculation;
use crate::types::CalcInput;

pub struct Processor {
    processed_count: u32,
}

impl Processor {
    pub fn new() -> Self {
        Processor { processed_count: 0 }
    }

    pub fn run(&self, input: &CalcInput) -> f64 {
        run_calculation(input)
    }

    pub fn run_batch(&self, inputs: &[CalcInput]) -> Vec<f64> {
        inputs.iter().map(|input| self.run(input)).collect()
    }

    pub fn count(&self) -> u32 {
        self.processed_count
    }

    fn track(&mut self) {
        self.processed_count += 1;
    }
}
