use super::int_code::Program;
use std::collections::{HashMap, HashSet, VecDeque};
#[derive(Debug, PartialEq)]
enum Tile {
    Floor,
    Wall,
    Oxygen,
}
#[derive(Copy, Clone)]
enum Dir {
    N,
    E,
    S,
    W,
}
struct Robot {
    program: Program,
    pos: (i32, i32),
}
impl Robot {
    fn move_dir(&mut self, dir: Dir) -> Tile {
        let input = match dir {
            Dir::N => 1,
            Dir::S => 2,
            Dir::W => 3,
            Dir::E => 4,
        };
        match self.program.run_input(Some(input)) {
            Some(0) => Tile::Wall,
            Some(1) => {
                self.pos = get_dir_coordinate(self.pos, dir);
                Tile::Floor
            }
            Some(2) => {
                self.pos = get_dir_coordinate(self.pos, dir);
                Tile::Oxygen
            }
            _ => unreachable!(),
        }
    }
}
fn create_map(robot: &mut Robot) -> HashMap<(i32, i32), Tile> {
    let mut map = HashMap::new();
    let mut backtrack = Vec::new();
    map.insert(robot.pos, Tile::Floor);
    loop {
        if let Some((unvisited, dir)) = [Dir::N, Dir::S, Dir::E, Dir::W]
            .iter()
            .map(|&d| (get_dir_coordinate(robot.pos, d), d))
            .find(|(new_pos, _)| !map.contains_key(&new_pos))
        {
            match robot.move_dir(dir) {
                tile @ Tile::Floor | tile @ Tile::Oxygen => {
                    backtrack.push(dir);
                    map.insert(unvisited, tile);
                }
                Tile::Wall => {
                    map.insert(unvisited, Tile::Wall);
                }
            }
        } else {
            if backtrack.is_empty() {
                break;
            }
            let prev_move = backtrack.pop().unwrap();
            let rev_dir = match prev_move {
                Dir::N => Dir::S,
                Dir::S => Dir::N,
                Dir::E => Dir::W,
                Dir::W => Dir::E,
            };
            robot.move_dir(rev_dir);
        }
    }
    map
}
fn get_dir_coordinate(start: (i32, i32), dir: Dir) -> (i32, i32) {
    match dir {
        Dir::N => (start.0, start.1 + 1),
        Dir::E => (start.0 + 1, start.1),
        Dir::S => (start.0, start.1 - 1),
        Dir::W => (start.0 - 1, start.1),
    }
}
fn search_map_for_oxygen(map: &HashMap<(i32, i32), Tile>) -> u32 {
    let mut visited = HashSet::new();
    visited.extend(map.iter().filter_map(|(pos, tile)| {
        if *tile == Tile::Wall {
            Some(pos)
        } else {
            None
        }
    }));
    let mut to_visit = VecDeque::new();
    to_visit.push_back(((0, 0), 0));
    while let Some((pos, steps)) = to_visit.pop_front() {
        visited.insert(pos);
        if let Some(Tile::Oxygen) = map.get(&pos) {
            return steps;
        }
        to_visit.extend(
            [Dir::N, Dir::S, Dir::E, Dir::W]
                .iter()
                .map(|d| (get_dir_coordinate(pos, *d), steps + 1))
                .filter(|(pos, _)| !visited.contains(pos)),
        );
    }
    panic!("oxygen not found");
}
fn map_oxygen_spread(
    start: (i32, i32),
    map: &HashMap<(i32, i32), Tile>,
) -> HashMap<(i32, i32), u32> {
    let mut visited = HashMap::new();
    visited.extend(map.iter().filter_map(|(pos, tile)| {
        if *tile == Tile::Wall {
            Some((*pos, 0))
        } else {
            None
        }
    }));
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start, 0));
    while let Some((pos, steps)) = to_visit.pop_front() {
        visited.insert(pos, steps);
        to_visit.extend(
            [Dir::N, Dir::S, Dir::E, Dir::W]
                .iter()
                .map(|d| (get_dir_coordinate(pos, *d), steps + 1))
                .filter(|(pos, _)| !visited.contains_key(pos)),
        );
    }
    visited
}

#[cfg(test)]
mod tests {
    use super::create_map;
    use super::map_oxygen_spread;
    use super::search_map_for_oxygen;
    use super::Program;
    use super::Robot;
    use super::Tile;
    use std::fs;
    #[test]
    fn day15_part1_and_2() {
        let code: Vec<i64> = fs::read_to_string("input/day15")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut robot = Robot { pos: (0, 0), program: Program::new(code) };
        let map = create_map(&mut robot);
        let ans1 = search_map_for_oxygen(&map);
        assert_eq!(ans1, 258);

        let oxygen_pos = map
            .iter()
            .find_map(|(pos, tile)| match tile {
                Tile::Oxygen => Some(pos),
                _ => None,
            })
            .unwrap();
        let oxygen_spread = map_oxygen_spread(*oxygen_pos, &map);
        let ans2 = oxygen_spread.values().max().unwrap();
        assert_eq!(*ans2, 372);
    }
}
