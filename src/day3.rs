use std::collections::HashMap;
fn find_intersect_distances(wires: &[&str]) -> (u32, u32) {
    let mut multi_wire_count: HashMap<(i32, i32), u32> = HashMap::new();
    let mut multi_wire_steps: HashMap<(i32, i32), u32> = HashMap::new();
    for wire in wires {
        let mut pos = (0, 0);
        let mut wire_map = HashMap::new();
        wire_map.insert((0, 0), 0);
        let mut count = 1;
        for instruction in wire.split(',') {
            draw_wire(instruction, &mut count, &mut wire_map, &mut pos);
        }
        for &pos in wire_map.keys() {
            *multi_wire_count.entry(pos).or_default() += 1;
        }
        for (pos, steps) in wire_map {
            *multi_wire_steps.entry(pos).or_default() += steps;
        }
    }

    let ans1 = multi_wire_count
        .iter()
        .filter(|(&pos, &count)| count > 1 && pos != (0, 0))
        .map(|((x, y), _)| x.abs() + y.abs())
        .min()
        .unwrap() as u32;
    let ans2 = multi_wire_steps
        .into_iter()
        .filter(|&(pos, _)| multi_wire_count[&pos] > 1 && pos != (0, 0))
        .map(|(_, steps)| steps)
        .min()
        .unwrap();
    (ans1, ans2)
}

fn draw_wire(
    draw: &str,
    count: &mut u32,
    map: &mut HashMap<(i32, i32), u32>,
    pos: &mut (i32, i32),
) {
    let steps: i32 = draw[1..].parse().unwrap();
    match &draw[0..1] {
        "R" => {
            for s in 1..=steps {
                map.entry((pos.0 + s, pos.1)).or_insert(*count);
                *count += 1;
            }
            *pos = (pos.0 + steps, pos.1);
        }
        "L" => {
            for s in 1..=steps {
                map.entry((pos.0 - s, pos.1)).or_insert(*count);
                *count += 1;
            }
            *pos = (pos.0 - steps, pos.1);
        }
        "U" => {
            for s in 1..=steps {
                map.entry((pos.0, pos.1 + s)).or_insert(*count);
                *count += 1;
            }
            *pos = (pos.0, pos.1 + steps);
        }
        "D" => {
            for s in 1..=steps {
                map.entry((pos.0, pos.1 - s)).or_insert(*count);
                *count += 1;
            }
            *pos = (pos.0, pos.1 - steps);
        }
        bad => panic!("{}", bad),
    }
}
#[cfg(test)]
mod tests {
    use super::find_intersect_distances;
    use std::fs;
    #[test]
    fn example_day3_part1_and_2() {
        let ans = find_intersect_distances(&["R8,U5,L5,D3", "U7,R6,D4,L4"]);
        assert_eq!(ans, (6, 30));
        let ans = find_intersect_distances(&[
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ]);
        assert_eq!(ans, (159, 610));
    }
    #[test]
    fn day3_part1_and_2() {
        let input: String = fs::read_to_string("input/day3").unwrap();
        let lines: Vec<&str> = input.lines().collect();
        let (ans1, ans2) = find_intersect_distances(&lines);
        assert_eq!(ans1, 352);
        assert_eq!(ans2, 43848);
    }
}
