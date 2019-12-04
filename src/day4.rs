use std::collections::HashMap;

fn brute_count(start: u32, end: u32) -> u32 {
    let mut count = 0;
    for n in start..=end {
        let digits: Vec<char> = n.to_string().chars().collect();
        let mut has_digit_twin = false;
        let is_growing = digits
            .windows(2)
            .inspect(|w| {
                if w[0] == w[1] {
                    has_digit_twin = true;
                }
            })
            .all(|w| w[0] <= w[1]);
        if has_digit_twin && is_growing {
            count += 1;
        }
    }
    count
}
fn brute_count_part2(start: u32, end: u32) -> u32 {
    let mut count = 0;
    'n: for n in start..=end {
        let digits: Vec<char> = n.to_string().chars().collect();
        let mut consecutive_count: HashMap<u32, u32> = HashMap::new();
        let mut consecutive = 1;
        for window in digits.windows(2) {
            if window[0] > window[1] {
                continue 'n;
            }
            if window[0] == window[1] {
                consecutive += 1;
            } else {
                *consecutive_count.entry(consecutive).or_default() += 1;
                consecutive = 1;
            }
        }
        *consecutive_count.entry(consecutive).or_default() += 1;
        if *consecutive_count.entry(2).or_default() > 0 {
            count += 1;
        }
    }
    count
}
#[cfg(test)]
mod tests {
    use super::brute_count;
    use super::brute_count_part2;
    #[test]
    fn example_day4_part1() {
        assert_eq!(brute_count(111_111, 111_111), 1);
        assert_eq!(brute_count(223_450, 223_450), 0);
    }
    #[test]
    fn day4_part1() {
        assert_eq!(brute_count(367_479, 893_698), 495);
    }
    #[test]
    fn example_day4_part2() {
        assert_eq!(brute_count_part2(111_122, 111_122), 1);
    }
    #[test]
    fn day4_part2() {
        assert_eq!(brute_count_part2(367_479, 893_698), 305);
    }
}
