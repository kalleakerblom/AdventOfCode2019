use std::collections::HashMap;
fn calculate_orbits(orbits: &[&str], start: &str, end: &str) -> (u32, u32) {
    let mut orbit_map = HashMap::new();
    for orb in orbits {
        let split: Vec<&str> = orb.split(')').collect();
        let (parent, child) = (split[0], split[1]);
        orbit_map.insert(child, parent);
    }
    //part 1
    let mut total_count = 0;
    let mut orbit_counts: HashMap<String, u32> = HashMap::new();
    orbit_counts.insert("COM".into(), 0);
    for child in orbit_map.keys() {
        total_count += get_count(child, &orbit_map, &mut orbit_counts);
    }
    //part 2
    let start_path = get_path(start, &orbit_map);
    let end_path = get_path(end, &orbit_map);
    let last_shared = start_path
        .iter()
        .zip(end_path.iter())
        .filter_map(|(sta, end)| if sta == end { Some(sta) } else { None })
        .last()
        .cloned()
        .unwrap_or_default();
    let transfers_needed = orbit_counts[start] - orbit_counts[&last_shared]
        + orbit_counts[end]
        - orbit_counts[&last_shared]
        - 2;
    (total_count, transfers_needed)
}
fn get_count(
    name: &str,
    orbit_map: &HashMap<&str, &str>,
    orbit_counts: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(count) = orbit_counts.get(name) {
        *count
    } else {
        let count = get_count(orbit_map[name], orbit_map, orbit_counts) + 1;
        orbit_counts.insert(name.into(), count);
        count
    }
}
fn get_path(name: &str, orbit_map: &HashMap<&str, &str>) -> Vec<String> {
    if let Some(parent) = orbit_map.get(name) {
        let mut path = get_path(parent, orbit_map);
        path.push(name.into());
        path
    } else {
        return vec![name.into()];
    }
}
#[cfg(test)]
mod tests {
    use super::calculate_orbits;
    use std::fs;
    #[test]
    fn example_day6_part1_and_2() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J",
            "J)K", "K)L",
        ];
        assert_eq!(calculate_orbits(&input, "L", "I"), (42, 3));
    }
    #[test]
    fn day6_part1() {
        let input_file = fs::read_to_string("input/day6").unwrap();
        let input: Vec<_> = input_file.trim().lines().collect();
        assert_eq!(calculate_orbits(&input, "YOU", "SAN"), (268_504, 409));
    }
}
