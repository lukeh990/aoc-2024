use std::{error, fs};

fn solve(input: &str) -> Result<i32, Box<dyn error::Error>> {
    Ok(0)
}

fn solve2(input: &str) -> Result<i32, Box<dyn error::Error>> {
    Ok(0)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = fs::read_to_string("input.txt")?;

    let output = solve(&input)?;
    println!("Solve 1: {output}");

    let output2 = solve2(&input)?;
    println!("Solve 2: {output2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_output() {
        let input = "";
        let expected_output = 0;

        let output = solve(input).unwrap();

        assert_eq!(expected_output, output);
    }

    #[test]
    fn sample_output2() {
        let input = "";
        let expected_output = 0;

        let output = solve2(input).unwrap();

        assert_eq!(expected_output, output);
    }
}
