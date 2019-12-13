use super::int_code::Program;
use std::collections::HashMap;

struct Robot {
    pos: (i32, i32),
    dir: (i32, i32),
    program: Program,
}
enum Turn {
    Left,
    Right,
}
#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}
impl Robot {
    fn new(program: Program) -> Self {
        Robot { dir: (0, 1), pos: (0, 0), program }
    }
    fn turn(&mut self, turn: Turn) {
        match turn {
            // (0,1) (-1,0) (0,-1) (1,0) (0,1)
            Turn::Left => self.dir = (-self.dir.1, self.dir.0),
            Turn::Right => self.dir = (self.dir.1, -self.dir.0),
        }
    }
    fn paint(&mut self, camera_input: Color) -> Option<Color> {
        let input = match camera_input {
            Color::Black => 0,
            Color::White => 1,
        };
        let paint = match self.program.run_input(Some(input))? {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("invalid paint code"),
        };
        let turn = match self.program.run_input(None)? {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("invalid turn code"),
        };
        self.turn(turn);
        Some(paint)
    }
    fn step(&mut self) {
        self.pos = (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1);
    }
}
fn print_paint(paint_map: &HashMap<(i32, i32), Color>) {
    use std::cmp;
    let mut min_x = i32::max_value();
    let mut max_x = i32::min_value();
    let mut min_y = i32::max_value();
    let mut max_y = i32::min_value();
    for &(x, y) in paint_map.keys() {
        min_x = cmp::min(min_x, x);
        min_y = cmp::min(min_y, y);
        max_x = cmp::max(max_x, x);
        max_y = cmp::max(max_y, y);
    }
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if let Some(Color::White) = paint_map.get(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::print_paint;
    use super::Color;
    use super::Program;
    use super::Robot;
    use super::Turn;
    use std::collections::HashMap;
    use std::fs;
    use std::iter;
    #[test]
    fn robot_turns() {
        let mut robot = Robot::new(Program::new(Vec::new()));
        assert_eq!(robot.dir, (0, 1));
        robot.turn(Turn::Left);
        assert_eq!(robot.dir, (-1, 0));
        robot.turn(Turn::Left);
        assert_eq!(robot.dir, (0, -1));
        robot.turn(Turn::Left);
        assert_eq!(robot.dir, (1, 0));
        robot.turn(Turn::Right);
        assert_eq!(robot.dir, (0, -1));
        robot.turn(Turn::Right);
        assert_eq!(robot.dir, (-1, 0));
        robot.turn(Turn::Right);
        assert_eq!(robot.dir, (0, 1));
    }
    #[test]
    fn day11_part1() {
        let mut code: Vec<i64> = fs::read_to_string("input/day11")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        code.extend(iter::repeat(0).take(1000));
        let prog = Program::new(code);
        let mut robot = Robot::new(prog);
        let mut painted = HashMap::new();
        let mut camera_input = Color::Black;
        while let Some(paint_color) = robot.paint(camera_input) {
            painted.insert(robot.pos, paint_color);
            robot.step();
            camera_input = *painted.get(&robot.pos).unwrap_or(&Color::Black);
        }
        assert_eq!(painted.len(), 2343);
    }
    #[test]
    fn day11_part2() {
        let mut code: Vec<i64> = fs::read_to_string("input/day11")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        code.extend(iter::repeat(0).take(1000));
        let prog = Program::new(code);
        let mut robot = Robot::new(prog);
        let mut painted = HashMap::new();
        let mut camera_input = Color::White;
        while let Some(paint_color) = robot.paint(camera_input) {
            painted.insert(robot.pos, paint_color);
            robot.step();
            camera_input = *painted.get(&robot.pos).unwrap_or(&Color::Black);
        }
        print_paint(&painted);
    }
}
