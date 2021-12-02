use std::fs;
use std::fs::read;

const FILENAME: &str = "resources/day1_input.txt";

pub fn part_1() -> i32 {
    return count(read_measurements())
}

pub fn part_2() -> i32 {
    let measurements = read_measurements()
        .as_slice()
        .windows(3)
        .map(|w| w.iter().sum())
        .collect();
    return count(measurements)
}

fn read_measurements() -> Vec<i32> {
    fs::read_to_string(FILENAME)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|measurement| measurement.trim().parse().unwrap())
        .collect()
}

fn count(measurements: Vec<i32>) -> i32 { // todo : make this as reciever so can be called after map()
    let mut increase_count = 0;
    for i in 0..(measurements.len() - 1) {
        if measurements[i] < measurements[i + 1] {
            increase_count += 1;
        }
    }
    return increase_count;
}

#[cfg(test)]
mod test {
    use crate::day1::{count, part_1, part_2};

    #[test]
    fn test_count_increases() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count(measurements), 7);
    }

    #[test]
    fn test_whole_day() {
        assert_eq!(part_1(), 1832);
        assert_eq!(part_2(), 1858);
    }
}

