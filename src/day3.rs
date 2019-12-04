use std::collections::HashMap;
fn find_intersect_distances(wire1: &str, wire2: &str) -> (u32, u32) {
    let mut wire_maps = Vec::new();
    for draw_instruction in &[wire1, wire2] {
        let wire_map = draw_wire(draw_instruction);
        wire_maps.push(wire_map);
    }
    let ans1 = wire_maps[0]
        .keys()
        .filter(|k| wire_maps[1].contains_key(k) && k != &&(0, 0))
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();
    let ans2 = wire_maps[0]
        .iter()
        .filter(|(pos, _)| pos != &&(0, 0))
        .filter_map(|(pos, steps)| {
            wire_maps[1].get(pos).map(|other_steps| steps + other_steps)
        })
        .min()
        .unwrap();
    (ans1 as u32, ans2)
}

fn draw_wire(draws: &str) -> HashMap<(i32, i32), u32> {
    let mut wire_map = HashMap::new();
    wire_map.insert((0, 0), 0);
    let mut pos = (0, 0);
    let mut count = 1;
    for draw in draws.split(',') {
        let steps: i32 = draw[1..].parse().unwrap();
        match &draw[0..1] {
            "R" => {
                for s in 1..=steps {
                    wire_map.entry((pos.0 + s, pos.1)).or_insert(count);
                    count += 1;
                }
                pos = (pos.0 + steps, pos.1);
            }
            "L" => {
                for s in 1..=steps {
                    wire_map.entry((pos.0 - s, pos.1)).or_insert(count);
                    count += 1;
                }
                pos = (pos.0 - steps, pos.1);
            }
            "U" => {
                for s in 1..=steps {
                    wire_map.entry((pos.0, pos.1 + s)).or_insert(count);
                    count += 1;
                }
                pos = (pos.0, pos.1 + steps);
            }
            "D" => {
                for s in 1..=steps {
                    wire_map.entry((pos.0, pos.1 - s)).or_insert(count);
                    count += 1;
                }
                pos = (pos.0, pos.1 - steps);
            }
            bad => panic!("{}", bad),
        }
    }
    wire_map
}
#[cfg(test)]
mod tests {
    use super::find_intersect_distances;
    use std::fs;
    #[test]
    fn example_day3_part1_and_2() {
        let ans = find_intersect_distances("R8,U5,L5,D3", "U7,R6,D4,L4");
        assert_eq!(ans, (6, 30));
        let ans = find_intersect_distances(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        );
        assert_eq!(ans, (159, 610));
    }
    #[test]
    fn day3_part1_and_2() {
        let input: String = fs::read_to_string("input/day3").unwrap();
        let lines: Vec<&str> = input.lines().collect();
        let (ans1, ans2) = find_intersect_distances(lines[0], lines[1]);
        assert_eq!(ans1, 352);
        assert_eq!(ans2, 43848);
    }
}
