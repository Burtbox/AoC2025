use std::fs;
use std::mem::replace;

fn solve<const N: usize>(input: &[&str]) -> u64 {
    let mut batteries = [0; N];

    input
        .iter()
        .map(|&bank| {
            // Start with enough batteries to make the bank, taken from the right of the input.
            let end = bank.len() - N;
            batteries.copy_from_slice(&bank.as_bytes()[end..]);

            // Scan from right to left, bubbling up any battery greater than the start of the bank.
            for mut next in bank[..end].bytes().rev() {
                for battery in &mut batteries {
                    if next < *battery {
                        break;
                    }
                    next = replace(battery, next);
                }
            }

            batteries.iter().fold(0, |joltage, &b| 10 * joltage + (b - b'0') as u64)
        })
        .sum()
}

// fn fetch_input() -> String {
//     let resp = reqwest::get("https://adventofcode.com/2025/day/3/input").await?;
//     let body = resp.text().await?;
//     println!("{body}");
//     body
// }

fn main() {
    let file_path = "./input.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    let lines: Vec<_> = contents.lines().collect();

    let result = solve::<12>(&lines);
    println!("Result: {}", result);
}
    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_a() {
        let result = solve::<12>(&["987654321111111"]);
        assert_eq!(result, 987654321111);
    }

        #[test]
    fn it_works_b() {
        let result = solve::<12>(&["811111111111119"]);
        assert_eq!(result, 811111111119);
    }

        #[test]
    fn it_works_c() {
        let result = solve::<12>(&["234234234234278"]);
        assert_eq!(result, 434234234278);
    }

        #[test]
    fn it_works_d() {
        let result = solve::<12>(&["818181911112111"]);
        assert_eq!(result, 888911112111);
    }
}
