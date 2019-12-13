use std::collections::HashMap;

fn draw_display(display: &HashMap<(i64, i64), i64>) {
    use std::cmp;
    let mut min_x = i64::max_value();
    let mut max_x = i64::min_value();
    let mut min_y = i64::max_value();
    let mut max_y = i64::min_value();
    for &(x, y) in display.keys() {
        min_x = cmp::min(min_x, x);
        min_y = cmp::min(min_y, y);
        max_x = cmp::max(max_x, x);
        max_y = cmp::max(max_y, y);
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match display.get(&(x, y)) {
                Some(1) => print!("|"),
                Some(2) => print!("#"),
                Some(3) => print!("_"),
                Some(4) => print!("o"),
                None | Some(_) => print!(" "),
            }
        }
        println!();
    }
}
#[cfg(test)]
mod tests {
    use super::draw_display;
    use crate::int_code::Program;
    use std::collections::HashMap;
    use std::fs;
    use std::iter;
    #[test]
    fn day13_part1() {
        let code: Vec<i64> = fs::read_to_string("input/day13")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .chain(iter::repeat(0).take(2000))
            .collect();
        let mut game = Program::new(code);
        let mut display = HashMap::new();
        while let (Some(x), Some(y), Some(id)) =
            (game.run(), game.run(), game.run())
        {
            display.insert((x, y), id);
        }
        let ans1 = display.values().filter(|&&id| id == 2).count();
        assert_eq!(ans1, 462);
    }
    #[test]
    fn day13_part2() {
        let mut code: Vec<i64> = fs::read_to_string("input/day13")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .chain(iter::repeat(0).take(2000))
            .collect();
        code[0] = 2;
        let mut game = Program::new(code);
        let mut display = HashMap::new();
        let mut score = 0;
        let mut paddle_x = 0;
        let mut ball_x = 0;
        loop {
            while let (Some(x), Some(y), Some(id)) =
                (game.run(), game.run(), game.run())
            {
                if (x, y) == (-1, 0) {
                    score = id;
                } else {
                    display.insert((x, y), id);
                    if id == 3 {
                        paddle_x = x;
                    }
                    if id == 4 {
                        ball_x = x;
                    }
                }
            }
            // draw_display(&display);
            use std::cmp::Ordering;
            let input = match paddle_x.cmp(&ball_x) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };
            game.set_input(Some(input));
            let blocks = display.values().filter(|&&id| id == 2).count();
            if blocks == 0 {
                break;
            }
        }
        assert_eq!(score, 23981);
    }
}
