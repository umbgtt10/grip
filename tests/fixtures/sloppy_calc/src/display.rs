pub fn format_result(result: f64, operation: &str) -> String {
    format!("{} = {}", operation, result)
}

fn format_result_to_string(result: f64, operation: &str, output: &mut String) {
    *output = format!("{} = {}", operation, result);
}

pub fn format_batch(results: &[f64]) -> String {
    let mut out = String::new();
    for (i, r) in results.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(&r.to_string());
    }
    out
}

fn print_summary(results: &[f64]) {
    let sum: f64 = results.iter().sum();
    println!("summary: sum={}, count={}", sum, results.len());
}
