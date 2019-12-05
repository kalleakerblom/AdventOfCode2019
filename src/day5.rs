#[derive(Debug)]
enum Param {
    Pos(usize),
    Im(i32),
}
#[derive(Debug)]
enum Op {
    Add([Param; 3]),
    Mul([Param; 3]),
    In(Param),
    Out(Param),
    JumpTrue([Param; 2]),
    JumpFalse([Param; 2]),
    Less([Param; 3]),
    Equal([Param; 3]),
    Halt,
}

fn run_program(code: &mut Vec<i32>, input: i32) -> Vec<i32> {
    let mut head = 0;
    let mut output = Vec::new();
    loop {
        let op = parse_op(&code, head);
        let value = |p: Param| match p {
            Param::Pos(pos) => code[pos],
            Param::Im(im) => im,
        };
        match op {
            Op::Add([p1, p2, Param::Pos(pos)]) => {
                code[pos] = value(p1) + value(p2);
                head += 4;
            }
            Op::Mul([p1, p2, Param::Pos(pos)]) => {
                code[pos] = value(p1) * value(p2);
                head += 4;
            }
            Op::In(Param::Pos(pos)) => {
                code[pos] = input;
                head += 2;
            }
            Op::Out(param) => {
                output.push(value(param));
                head += 2;
            }
            Op::JumpTrue([p1, p2]) => {
                if value(p1) != 0 {
                    head = value(p2) as usize;
                } else {
                    head += 3;
                }
            }
            Op::JumpFalse([p1, p2]) => {
                if value(p1) == 0 {
                    head = value(p2) as usize;
                } else {
                    head += 3;
                }
            }
            Op::Less([p1, p2, Param::Pos(pos)]) => {
                code[pos] = if value(p1) < value(p2) { 1 } else { 0 };
                head += 4;
            }
            Op::Equal([p1, p2, Param::Pos(pos)]) => {
                code[pos] = if value(p1) == value(p2) { 1 } else { 0 };
                head += 4;
            }
            Op::Halt => return output,
            _ => panic!("bad op"),
        }
    }
}
fn parse_op(code: &[i32], head: usize) -> Op {
    let op_code = code[head];
    let de = op_code % 100;
    let op_code = op_code / 100;
    let c = op_code % 10;
    let b = (op_code / 10) % 10;
    let a = (op_code / 100) % 10;
    let make_param = |val, mode| match mode {
        0 => Param::Pos(val as usize),
        1 => Param::Im(val),
        _ => panic!(),
    };
    match de {
        99 => Op::Halt,
        1 => Op::Add([
            make_param(code[head + 1], c),
            make_param(code[head + 2], b),
            make_param(code[head + 3], a),
        ]),
        2 => Op::Mul([
            make_param(code[head + 1], c),
            make_param(code[head + 2], b),
            make_param(code[head + 3], a),
        ]),
        3 => Op::In(make_param(code[head + 1], c)),
        4 => Op::Out(make_param(code[head + 1], c)),
        5 => Op::JumpTrue([
            make_param(code[head + 1], c),
            make_param(code[head + 2], b),
        ]),
        6 => Op::JumpFalse([
            make_param(code[head + 1], c),
            make_param(code[head + 2], b),
        ]),
        7 => Op::Less([
            make_param(code[head + 1], c),
            make_param(code[head + 2], b),
            make_param(code[head + 3], a),
        ]),
        8 => Op::Equal([
            make_param(code[head + 1], c),
            make_param(code[head + 2], b),
            make_param(code[head + 3], a),
        ]),

        bad => panic!("bad op code ({}) at ({})", bad, head),
    }
}
#[cfg(test)]
mod tests {
    use super::run_program;
    use std::fs;
    #[test]
    fn example_day5_part1() {
        let mut program = vec![1002, 4, 3, 4, 33];
        assert_eq!(run_program(&mut program, 1), vec![]);
        assert_eq!(program, vec![1002, 4, 3, 4, 99]);
    }
    #[test]
    fn day5_part1_and_2() {
        let mut program: Vec<i32> = fs::read_to_string("input/day5")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        //part 1
        assert_eq!(
            run_program(&mut program.clone(), 1),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 15259545]
        );
        //part 2
        assert_eq!(run_program(&mut program, 5), vec![7616021]);
    }
}
