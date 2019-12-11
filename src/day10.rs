use num::integer::gcd;
use std::collections::HashSet;
fn parse_asteroids(input: &str) -> HashSet<(i32, i32)> {
    let mut map = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((x as i32, y as i32));
            }
        }
    }
    map
}

fn find_station(map: &HashSet<(i32, i32)>) -> ((i32, i32), usize) {
    let mut best_base = (0, 0);
    let mut max_los = 0;
    for base in map {
        let mut seen = HashSet::new();
        for target in map {
            if target == base {
                continue;
            }
            let dir = (target.0 - base.0, target.1 - base.1);
            seen.insert(reduce_dir(dir));
        }
        let los = seen.len();
        if los > max_los {
            max_los = los;
            best_base = *base;
        }
    }
    (best_base, max_los)
}
fn reduce_dir(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, 0) => (0, 0),
        (0, y) if y < 0 => (0, -1),
        (0, y) if y > 0 => (0, 1),
        (x, 0) if x < 0 => (-1, 0),
        (x, 0) if x > 0 => (1, 0),
        (x, y) => {
            let div = gcd(x, y);
            (x / div, y / div)
        }
    }
}
fn shoot_asteroids(base:(i32,i32),mut map:HashSet<(i32,i32)>,shots:u32)->(i32,i32)
{
    
    
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::find_station;
    use super::parse_asteroids;
    use std::fs;
    #[test]
    fn day10_part1() {
        let input = fs::read_to_string("input/day10").unwrap();
        let asteroids = parse_asteroids(&input);
        let ans = find_station(&asteroids);
        assert_eq!(ans.1, 280);
    }
}
