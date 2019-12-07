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
fn run_program(
    head: &mut usize,
    code: &mut Vec<i32>,
    mut input: Vec<i32>,
) -> Option<i32> {
    loop {
        let op = parse_op(&code, *head);
        let value = |p: Param| match p {
            Param::Pos(pos) => code[pos],
            Param::Im(im) => im,
        };
        match op {
            Op::Add([p1, p2, Param::Pos(pos)]) => {
                code[pos] = value(p1) + value(p2);
                *head += 4;
            }
            Op::Mul([p1, p2, Param::Pos(pos)]) => {
                code[pos] = value(p1) * value(p2);
                *head += 4;
            }
            Op::In(Param::Pos(pos)) => {
                code[pos] = input.pop().expect("missing input");
                *head += 2;
            }
            Op::Out(param) => {
                *head += 2;
                return Some(value(param));
            }
            Op::JumpTrue([p1, p2]) => {
                if value(p1) != 0 {
                    *head = value(p2) as usize;
                } else {
                    *head += 3;
                }
            }
            Op::JumpFalse([p1, p2]) => {
                if value(p1) == 0 {
                    *head = value(p2) as usize;
                } else {
                    *head += 3;
                }
            }
            Op::Less([p1, p2, Param::Pos(pos)]) => {
                code[pos] = if value(p1) < value(p2) { 1 } else { 0 };
                *head += 4;
            }
            Op::Equal([p1, p2, Param::Pos(pos)]) => {
                code[pos] = if value(p1) == value(p2) { 1 } else { 0 };
                *head += 4;
            }
            Op::Halt => return None,
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
fn run_program_chain(
    input: i32,
    heads: [&mut usize; 5],
    amps: [&mut Vec<i32>; 5],
) -> Option<i32> {
    let a_out = run_program(heads[0], amps[0], vec![input])?;
    let b_out = run_program(heads[1], amps[1], vec![a_out])?;
    let c_out = run_program(heads[2], amps[2], vec![b_out])?;
    let d_out = run_program(heads[3], amps[3], vec![c_out])?;
    run_program(heads[4], amps[4], vec![d_out])
}
#[cfg(test)]
mod tests {
    use super::run_program;
    use super::run_program_chain;
    use itertools::Itertools;
    use std::cmp;
    use std::fs;
    #[test]
    fn day7_part1() {
        let amp_code: Vec<i32> = fs::read_to_string("input/day7")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let mut max_out = 0;
        for set in (0..5).permutations(5) {
            let out_a =
                run_program(&mut 0, &mut amp_code.clone(), vec![0, set[0]])
                    .unwrap();
            let out_b =
                run_program(&mut 0, &mut amp_code.clone(), vec![out_a, set[1]])
                    .unwrap();
            let out_c =
                run_program(&mut 0, &mut amp_code.clone(), vec![out_b, set[2]])
                    .unwrap();
            let out_d =
                run_program(&mut 0, &mut amp_code.clone(), vec![out_c, set[3]])
                    .unwrap();
            let out_e =
                run_program(&mut 0, &mut amp_code.clone(), vec![out_d, set[4]])
                    .unwrap();
            max_out = cmp::max(max_out, out_e);
        }
        assert_eq!(max_out, 914828);
    }
    #[test]
    fn day7_part2() {
        let amp_code: Vec<i32> = fs::read_to_string("input/day7")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut max_out = 0;

        for set in (5..10).permutations(5) {
            let mut amp_a = amp_code.clone();
            let mut amp_b = amp_code.clone();
            let mut amp_c = amp_code.clone();
            let mut amp_d = amp_code.clone();
            let mut amp_e = amp_code.clone();
            let mut head_a = 0;
            let mut head_b = 0;
            let mut head_c = 0;
            let mut head_d = 0;
            let mut head_e = 0;
            let out_a =
                run_program(&mut head_a, &mut amp_a, vec![0, set[0]]).unwrap();
            let out_b =
                run_program(&mut head_b, &mut amp_b, vec![out_a, set[1]])
                    .unwrap();
            let out_c =
                run_program(&mut head_c, &mut amp_c, vec![out_b, set[2]])
                    .unwrap();
            let out_d =
                run_program(&mut head_d, &mut amp_d, vec![out_c, set[3]])
                    .unwrap();
            let out_e =
                run_program(&mut head_e, &mut amp_e, vec![out_d, set[4]])
                    .unwrap();
            max_out = cmp::max(out_e, max_out);
            let mut input = out_e;
            loop {
                if let Some(output) = run_program_chain(
                    input,
                    [
                        &mut head_a,
                        &mut head_b,
                        &mut head_c,
                        &mut head_d,
                        &mut head_e,
                    ],
                    [
                        &mut amp_a, &mut amp_b, &mut amp_c, &mut amp_d,
                        &mut amp_e,
                    ],
                ) {
                    input = output;
                    max_out = cmp::max(output, max_out);
                } else {
                    break;
                }
            }
        }
        dbg!(max_out);
    }
}
