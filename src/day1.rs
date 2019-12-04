fn total_fuel_count(masses: &[u32]) -> u32 {
    masses.iter().map(|m| m / 3 - 2).sum()
}

fn recursive_fuel_count(mass: u32) -> u32 {
    if mass <= 2 * 3 {
        return 0;
    }
    let count = mass / 3 - 2;
    count + recursive_fuel_count(count)
}
fn total_recursive_fuel_count(masses: &[u32]) -> u32 {
    masses.iter().map(|m| recursive_fuel_count(*m)).sum()
}
#[cfg(test)]
mod tests {
    use super::total_fuel_count;
    use super::total_recursive_fuel_count;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    #[test]
    fn example_day1_part1() {
        assert_eq!(total_fuel_count(&[100_756]), 33583);
    }
    #[test]
    fn day_1_part1() {
        let buf = BufReader::new(File::open("input/day1").unwrap());
        let masses: Vec<u32> =
            buf.lines().map(|s| s.unwrap().parse::<u32>().unwrap()).collect();
        assert_eq!(total_fuel_count(&masses), 3_372_463);
    }
    #[test]
    fn example_day1_part2() {
        assert_eq!(total_recursive_fuel_count(&[100_756]), 50346);
    }
    #[test]
    fn day_1_part2() {
        let buf = BufReader::new(File::open("input/day1").unwrap());
        let masses: Vec<u32> =
            buf.lines().map(|s| s.unwrap().parse::<u32>().unwrap()).collect();
        assert_eq!(total_recursive_fuel_count(&masses), 5_055_835);
    }
}
