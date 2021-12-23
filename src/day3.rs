use std::borrow::Borrow;
use std::detect::__is_feature_detected::xsave;
use std::fs;
use std::fs::read;
use std::slice::SliceIndex;
use std::str::Lines;

const FILENAME: &str = "resources/day3_input.txt";

pub fn part_1() -> i32 {

    let input = fs::read_to_string(FILENAME)
        .expect("Something went wrong reading the file");

    let lines = input.lines();
    let line_count = get_lines_length(&lines);
    let compiled_diagnostics = lines.map(|entry|
        entry.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>()
    ).reduce(|transformed, next|
            transformed.into_iter()
                .zip(next.into_iter())
            .map(|(x,y)| x + y)
                .collect()
        ).unwrap();

    let epsilon_rate = get_epsilon_rate(&compiled_diagnostics, &line_count);
    let gamma_rate = get_gamma_rate(&compiled_diagnostics, &line_count);
    println!("epsilon = {}, gamma =  {}",epsilon_rate, gamma_rate);

    return epsilon_rate * gamma_rate
}
// todo simplify with get_gamma_rate
fn get_epsilon_rate(compiled_diagnostics: &Vec<i32>, line_count: &usize) -> i32 {
    return compiled_diagnostics
        .clone()
        .into_iter()
        .map(|count | if (*line_count as i32) - count > count { 1 } else { 0 })
        .fold(0,|acc, binary| acc*2 + binary)
}

fn get_gamma_rate(compiled_diagnostics: &Vec<i32>, line_count: &usize) -> i32 {
   return compiled_diagnostics
       .clone()
       .into_iter()
       .map(|count | if (*line_count as i32) - count < count { 1 } else { 0 })
       .fold(0,|acc, binary| acc*2 + binary)
}

fn get_lines_length(x : &Lines) -> usize {
    return x.clone().count()
}

pub fn part_2() -> i32 {
    return -1
}

#[cfg(test)]
mod test {
    use crate::day3::part_1;

    #[test]
    pub fn test_part_1() {
        assert_eq!(1071734, part_1());
    }
}
