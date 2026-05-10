use crate::ops::run_calculation;
use crate::types::CalcInput;

pub struct Processor {
    processed_count: u32,
}

impl Processor {
    pub fn new() -> Self {
        Processor { processed_count: 0 }
    }

    pub fn process(&mut self, input: &CalcInput) -> f64 {
        let result = run_calculation(input);
        self.processed_count += 1;
        println!("processed input #{}: {}", self.processed_count, result);
        result
    }

    pub fn process_batch(&mut self, inputs: &[CalcInput]) -> Vec<f64> {
        let mut results = Vec::new();
        for input in inputs {
            let r = self.process(input);
            results.push(r);
        }
        println!("batch complete: {} items", inputs.len());
        results
    }

    pub fn count(&self) -> u32 {
        self.processed_count
    }

    fn reset(&mut self) {
        self.processed_count = 0;
        println!("processor reset");
    }
}
