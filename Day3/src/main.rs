use reqwest::header::COOKIE;
use std::env;

fn solve(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let digits: Vec<i32> = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as i32)
                .collect();

            if digits.len() < 2 {
                return None;
            }

            let trimmed = &digits[..digits.len() - 1];
            let (idx, &largest) = trimmed
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|&(_, v)| v)
                .unwrap();

            let right = &digits[idx + 1..];
            let second = *right.iter().max().unwrap();

            Some(largest * 10 + second)
        })
        .sum()
}

async fn fetch_input() -> Result<String, reqwest::Error> {
    let session = env::var("AOC_SESSION")
        .expect("Set the AOC_SESSION environment variable with your AoC session token");

    let client = reqwest::Client::new();
    let response = client
        .get("https://adventofcode.com/2025/day/3/input")
        .header(COOKIE, format!("session={session}"))
        .send()
        .await?
        .error_for_status()?;

    response.text().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fetch_input().await?;
    let result = solve(&input);
    println!("Result: {result}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn it_works_a() {
        assert_eq!(solve("987654321111111"), 98);
    }

    #[test]
    fn it_works_b() {
        assert_eq!(solve("811111111111119"), 89);
    }

    #[test]
    fn it_works_c() {
        assert_eq!(solve("234234234234278"), 78);
    }

    #[test]
    fn it_works_d() {
        assert_eq!(solve("818181911112111"), 92);
    }
}
