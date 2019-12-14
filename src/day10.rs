use num::integer::gcd;
use std::collections::HashMap;
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
fn shoot_asteroids(
    base: (i32, i32),
    mut map: HashSet<(i32, i32)>,
    shots: u32,
) -> (i32, i32) {
    map.remove(&base);
    let mut fired = 0;
    loop {
        let mut shoot_dir_map: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        for target in &map {
            let dir = (target.0 - base.0, target.1 - base.1);
            let reduced = reduce_dir(dir);
            shoot_dir_map
                .entry(reduced)
                .and_modify(|prev| {
                    if (dir.0.abs(), dir.1.abs()) < (prev.0.abs(), prev.1.abs())
                    {
                        *prev = dir;
                    }
                })
                .or_insert(dir);
        }
        let mut to_shoot_dirs: Vec<(i32, i32)> =
            shoot_dir_map.values().cloned().collect();
        use std::f32;
        let clock = |x: f32, y: f32| {
            (x.atan2(-y) + 2. * f32::consts::PI) % (2. * f32::consts::PI)
        };
        to_shoot_dirs.sort_by(|a, b| {
            clock(a.0 as f32, a.1 as f32)
                .partial_cmp(&clock(b.0 as f32, b.1 as f32))
                .unwrap()
        });
        let to_shoot: Vec<_> = to_shoot_dirs
            .iter()
            .map(|(dx, dy)| (base.0 + dx, base.1 + dy))
            .collect();
        for target in to_shoot {
            map.remove(&target);
            fired += 1;
            if fired == shots {
                return target;
            }
        }
        if map.is_empty() {
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::find_station;
    use super::parse_asteroids;
    use super::shoot_asteroids;
    use std::fs;
    #[test]
    fn day10_part1_and_2() {
        let input = fs::read_to_string("input/day10").unwrap();
        let asteroids = parse_asteroids(&input);
        let ans = find_station(&asteroids);
        assert_eq!(ans.1, 280);
        let base = ans.0;
        let ans2 = shoot_asteroids(base, asteroids, 200);
        assert_eq!(ans2, (7, 6));
    }

    #[test]
    fn day10_example2() {
        let input = fs::read_to_string("input/example10").unwrap();
        let asteroids = parse_asteroids(&input);
        let ans = find_station(&asteroids);
        let base = ans.0;
        let ans2 = shoot_asteroids(base, asteroids, 200);
        assert_eq!(ans2, (8, 2));
    }
}
