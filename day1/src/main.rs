use std::{collections::HashMap, error, fs, str::FromStr};

fn some_and_parse<T: FromStr>(option: Option<&&str>) -> Result<T, Box<dyn error::Error>> {
    if let Some(value) = option {
        match value.parse::<T>() {
            Ok(result) => Ok(result),
            Err(_) => Err(Box::from("Parse Error!")),
        }
    } else {
        Err(Box::from("No value in option"))
    }
}

fn parse_lists(input: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn error::Error>> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.lines() {
        let split: Vec<&str> = line.split("   ").collect();

        if split.len() != 2 {
            return Err(Box::from("Input must be in pairs of 2!"));
        }

        left.push(some_and_parse(split.first())?);

        right.push(some_and_parse(split.get(1))?);
    }

    Ok((left, right))
}

fn solve(input: &str) -> Result<i32, Box<dyn error::Error>> {
    let (mut left, mut right) = parse_lists(input)?;
    left.sort();
    right.sort();

    let mut total = 0;

    for i in 0..left.len() {
        let left_opt = left.get(i);
        let right_opt = right.get(i);

        if let (Some(left_item), Some(right_item)) = (left_opt, right_opt) {
            let dist = (left_item - right_item).abs();
            total += dist;
        }
    }

    Ok(total)
}

fn solve2(input: &str) -> Result<i32, Box<dyn error::Error>> {
    let (left, right) = parse_lists(input)?;

    let mut map: HashMap<i32, i32> = HashMap::new();

    for right_item in right.iter() {
        let mut current_value = match map.get(right_item) {
            None => 0,
            Some(value) => *value,
        };

        current_value += 1;

        map.insert(*right_item, current_value);
    }

    let mut score = 0;

    for left_item in left.iter() {
        let instances = match map.get(left_item) {
            None => 0,
            Some(value) => *value,
        };

        score += *left_item * instances;
    }

    Ok(score)
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
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let expected_output = 11;

        let output = solve(input).unwrap();

        assert_eq!(expected_output, output);
    }

    #[test]
    fn sample_output2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let expected_output = 31;

        let output = solve2(input).unwrap();

        assert_eq!(expected_output, output);
    }
}
