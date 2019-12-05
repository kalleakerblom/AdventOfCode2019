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

fn run_program(mut code: Vec<i32>, input: i32) -> Vec<i32> {
    let mut head = 0;
    loop {
        let op = parse_op(&code, head);
        let get_val = |p: Param| match p {
            Param::Pos(pos) => code[pos],
            Param::Im(im) => im,
        };
        match op {
            Op::Add([p1, p2, Param::Pos(pos)]) => {
                code[pos] = get_val(p1) + get_val(p2);
                head += 4;
            }
            Op::Mul([p1, p2, Param::Pos(pos)]) => {
                code[pos] = get_val(p1) * get_val(p2);
                head += 4;
            }
            Op::In(Param::Pos(pos)) => {
                code[pos] = input;
                head += 2;
            }
            Op::Out(param) => {
                println!("out:{}", get_val(param));
                head += 2;
            }
            Op::JumpTrue([p1, p2]) => {
                if get_val(p1) != 0 {
                    head = get_val(p2) as usize;
                } else {
                    head += 3;
                }
            }
            Op::JumpFalse([p1, p2]) => {
                if get_val(p1) == 0 {
                    head = get_val(p2) as usize;
                } else {
                    head += 3;
                }
            }
            Op::Less([p1, p2, Param::Pos(pos)]) => {
                code[pos] = if get_val(p1) < get_val(p2) { 1 } else { 0 };
                head += 4;
            }
            Op::Equal([p1, p2, Param::Pos(pos)]) => {
                code[pos] = if get_val(p1) == get_val(p2) { 1 } else { 0 };
                head += 4;
            }
            Op::Halt => return code,
            _ => panic!("bad op"),
        }
    }
}
fn parse_op(code: &[i32], head: usize) -> Op {
    let op_code = code[head];
    let DE = op_code % 100;
    let op_code = op_code / 100;
    let C = op_code % 10;
    let B = (op_code / 10) % 10;
    let A = (op_code / 100) % 10;
    let build_param = |val, mode| match mode {
        0 => Param::Pos(val as usize),
        1 => Param::Im(val),
        _ => panic!(),
    };
    match DE {
        99 => Op::Halt,
        1 => Op::Add([
            build_param(code[head + 1], C),
            build_param(code[head + 2], B),
            build_param(code[head + 3], A),
        ]),
        2 => Op::Mul([
            build_param(code[head + 1], C),
            build_param(code[head + 2], B),
            build_param(code[head + 3], A),
        ]),
        3 => Op::In(build_param(code[head + 1], C)),
        4 => Op::Out(build_param(code[head + 1], C)),
        5 => Op::JumpTrue([
            build_param(code[head + 1], C),
            build_param(code[head + 2], B),
        ]),
        6 => Op::JumpFalse([
            build_param(code[head + 1], C),
            build_param(code[head + 2], B),
        ]),
        7 => Op::Less([
            build_param(code[head + 1], C),
            build_param(code[head + 2], B),
            build_param(code[head + 3], A),
        ]),
        8 => Op::Equal([
            build_param(code[head + 1], C),
            build_param(code[head + 2], B),
            build_param(code[head + 3], A),
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
        assert_eq!(
            run_program(vec![1002, 4, 3, 4, 33], 1),
            vec![1002, 4, 3, 4, 99]
        )
    }
    #[test]
    fn day5_part1_and_2() {
        let program: Vec<i32> = fs::read_to_string("input/day5")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        //part 1
        run_program(program.clone(), 1);
        //part 2
        run_program(program, 5);
    }
}
