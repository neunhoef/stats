use std::io::{self, BufRead};

#[derive(Debug)]
struct Statistics {
    count: usize,
    min: f64,
    max: f64,
    average: f64,
    median: f64,
    percentile_90: f64,
    percentile_99: f64,
}

fn calculate_statistics(numbers: &[f64]) -> Option<Statistics> {
    let count = numbers.len();

    if count == 0 {
        return None;
    }

    // Create a sorted copy of the numbers
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Min and max
    let min = sorted[0];
    let max = sorted[count - 1];

    // Average
    let sum: f64 = sorted.iter().sum();
    let average = sum / count as f64;

    // Median
    let median = if count % 2 == 0 {
        (sorted[count / 2 - 1] + sorted[count / 2]) / 2.0
    } else {
        sorted[count / 2]
    };

    // Percentiles
    // For 90th percentile, we want the value at index floor(n * 0.9)
    let index_90 = (count as f64 * 0.9).floor() as usize;
    let percentile_90 = sorted[index_90];

    // For 99th percentile, we want the value at index floor(n * 0.99)
    let index_99 = (count as f64 * 0.99).floor() as usize;
    let percentile_99 = sorted[index_99];

    Some(Statistics {
        count,
        min,
        max,
        average,
        median,
        percentile_90,
        percentile_99,
    })
}

fn main() {
    println!(
        "Enter numbers (one per line). Press Ctrl+D (Unix) or Ctrl+Z (Windows) when finished."
    );

    let stdin = io::stdin();
    let mut numbers = Vec::new();

    for line in stdin.lock().lines() {
        match line {
            Ok(input) => match input.trim().parse::<f64>() {
                Ok(number) => numbers.push(number),
                Err(_) => {
                    if !input.trim().is_empty() {
                        eprintln!("Warning: Couldn't parse '{}' as a number. Skipping.", input);
                    }
                }
            },
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }

    match calculate_statistics(&numbers) {
        Some(stats) => {
            println!("\nStatistics Summary");
            println!("===================");
            println!("Count:           {}", stats.count);
            println!("Minimum:         {:.6}", stats.min);
            println!("Maximum:         {:.6}", stats.max);
            println!("Average:         {:.6}", stats.average);
            println!("Median:          {:.6}", stats.median);
            println!("90th Percentile: {:.6}", stats.percentile_90);
            println!("99th Percentile: {:.6}", stats.percentile_99);
        }
        None => {
            println!("No valid numbers were provided.");
        }
    }
}
