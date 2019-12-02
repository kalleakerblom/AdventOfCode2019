fn run_program(mut code: Vec<usize>) -> Vec<usize> {
    let mut head = 0;
    loop {
        let op = code[head];
        if op == 99 {
            return code;
        }
        let pos_a = code[head + 1];
        let pos_b = code[head + 2];
        let pos_res = code[head + 3];
        match op {
            1 => code[pos_res] = code[pos_a] + code[pos_b],
            2 => code[pos_res] = code[pos_a] * code[pos_b],
            _ => panic!("Unknown op code!"),
        }
        head += 4;
    }
}
#[cfg(test)]
mod tests {
    use super::run_program;
    use std::fs;

    #[test]
    fn example_day2_part1() {
        assert_eq!(
            run_program(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
        assert_eq!(
            run_program(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(run_program(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(run_program(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    }
    #[test]
    fn day2_part1() {
        let mut program: Vec<usize> = fs::read_to_string("input/day2")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        program[1] = 12;
        program[2] = 2;
        let ans = run_program(program);
        assert_eq!(ans[0], 5_110_675);
    }
    #[test]
    fn day2_part2() {
        let program: Vec<usize> = fs::read_to_string("input/day2")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        for p1 in 0..99 {
            for p2 in 0..99 {
                let mut program = program.clone();
                program[1] = p1;
                program[2] = p2;
                let ans = run_program(program);
                if ans[0] == 19_690_720 {
                    assert_eq!((p1, p2), (48, 47));
                    return;
                }
            }
        }
        panic!("p1,p2 not found");
    }
}
