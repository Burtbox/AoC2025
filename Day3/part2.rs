use std::fs;

fn solve(input: String) -> i64 {
    // println!("Processing: {:?}", input);

    let mut acc = 0;
    
    for line in input.lines() {

        let vector_of_ints: Vec<i64> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i64)
        .collect();
        
        let mut start = 0;
        
        for digit in 0..12 {
            let limit = 11 - digit;
            let Some((pos, &next_digit)) = vector_of_ints[start..vector_of_ints.len() - limit].to_vec().iter().enumerate().rev().max_by(|a,b| a.1.cmp(b.1)) else { panic!("bad times friend") };

            start += pos+1;
            acc = acc * 10 + next_digit; 
        }
    }

    acc
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

    let result = solve(contents);
    println!("Result: {}", result);
}
    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_a() {
        let result = solve("987654321111111".to_string());
        assert_eq!(result, 987654321111);
    }

        #[test]
    fn it_works_b() {
        let result = solve("811111111111119".to_string());
        assert_eq!(result, 811111111119);
    }

        #[test]
    fn it_works_c() {
        let result = solve("234234234234278".to_string());
        assert_eq!(result, 434234234278);
    }

        #[test]
    fn it_works_d() {
        let result = solve("818181911112111".to_string());
        assert_eq!(result, 888911112111);
    }
}
