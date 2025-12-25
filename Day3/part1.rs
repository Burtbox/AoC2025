use std::fs;

fn solve(input: u64) -> u64 {
    // TODO: implement your solution
    println!("Processing: {}", input);
    98 // placeholder
}

fn main() {

    let file_path = "./exampleIn.txt";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let result = solve(987654321111111);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_worksA() {
        let result = solve(987654321111111);
        assert_eq!(result, 98);
    }

        #[test]
    fn it_worksB() {
        let result = solve(811111111111119);
        assert_eq!(result, 89);
    }

        #[test]
    fn it_worksC() {
        let result = solve(234234234234278);
        assert_eq!(result, 78);
    }

        #[test]
    fn it_worksD() {
        let result = solve(818181911112111);
        assert_eq!(result, 92);
    }
}
