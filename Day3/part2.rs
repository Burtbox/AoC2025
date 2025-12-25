use reqwest::Error;

fn solve(input: String) -> i32 {
    println!("Processing: {:?}", input);

    let mut acc = 0;

    //let first_line = input.lines().nth(0).unwrap();

    for line in input.lines() {
        let vector_of_ints: Vec<i32> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();
        
        let trimmed = &vector_of_ints[..vector_of_ints.len() - 1];
        
        println!("Digits: {:?}", trimmed);
        
        let (idx, &largest) = trimmed
        .iter()
        .enumerate().rev()
        .max_by_key(|&(_, v)| v)
        .unwrap();
        
        println!("With largest num: {largest} at {idx}");  
        
        let right_ints = &vector_of_ints[idx + 1..];
        
        println!("right ints {:?}", right_ints);
        
        let second_largest = right_ints.iter().max().unwrap();
        
        println!("With second largest num: {second_largest}"); 
        
        let add_on = largest * 10 + second_largest;
        
        println!("comibned largest num: {add_on}"); 

        acc += add_on
    }

    acc
}

fn fetch_input() -> String {
    let resp = reqwest::get("https://adventofcode.com/2025/day/3/input").await?;
    let body = resp.text().await?;
    println!("{body}");
    body
}

fn main() {
    let input = fetch_input();
    let result = solve(input);
    println!("Result: {}", result);
}
    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_a() {
        let result = solve("987654321111111".to_string());
        assert_eq!(result, 98);
    }

        #[test]
    fn it_works_b() {
        let result = solve("811111111111119".to_string());
        assert_eq!(result, 89);
    }

        #[test]
    fn it_works_c() {
        let result = solve("234234234234278".to_string());
        assert_eq!(result, 78);
    }

        #[test]
    fn it_works_d() {
        let result = solve("818181911112111".to_string());
        assert_eq!(result, 92);
    }
}
