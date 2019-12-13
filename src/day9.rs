#[cfg(test)]
mod tests {
    use crate::int_code::Program;
    use std::fs;
    use std::iter;
    #[test]
    fn day9_example_1() {
        let mut code: Vec<i64> =
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
        code.extend(iter::repeat(0).take(100));
        let mut prog = Program::new(code);
        while let Some(out) = prog.run_input(None) {
            println!("{:?}", out)
        }

        let mut code: Vec<i64> = "1102,34915192,34915192,7,4,7,99,0"
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        code.extend(iter::repeat(0).take(100));
        let mut prog = Program::new(code);
        assert_eq!(prog.run_input(None), Some(1_219_070_632_396_864));

        let mut code: Vec<i64> = "104,1125899906842624,99"
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        code.extend(iter::repeat(0).take(100));
        let mut prog = Program::new(code);
        assert_eq!(prog.run_input(None), Some(1_125_899_906_842_624));
    }
    #[test]
    fn day9_part1_and_2() {
        let mut code: Vec<i64> = fs::read_to_string("input/day9")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        code.extend(iter::repeat(0).take(1000));
        // part 1
        let mut prog = Program::new(code.clone());
        assert_eq!(prog.run_input(Some(1)), Some(3_742_852_857));
        // part 2
        let mut prog_2 = Program::new(code);
        assert_eq!(prog_2.run_input(Some(2)), Some(73439));
    }
}
