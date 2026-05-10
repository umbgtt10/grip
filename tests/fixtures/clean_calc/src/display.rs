pub fn format_result(result: f64, operation: &str) -> String {
    format!("{} = {}", operation, result)
}

pub fn format_batch(results: &[f64]) -> String {
    results
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn format_summary(results: &[f64]) -> String {
    let sum: f64 = results.iter().sum();
    format!("sum={}, count={}", sum, results.len())
}

fn wrap_in_brackets(text: &str) -> String {
    format!("[{}]", text)
}
