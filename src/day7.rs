#[derive(Debug)]
enum Param {
    Pos(usize),
    Rel(i32),
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
    OffsetBase(Param),
    Halt,
}
struct Program {
    head: usize,
    code: Vec<i32>,
    base: usize,
}
impl Program {
    fn new(head: usize, code: Vec<i32>, base: usize) -> Self {
        Program { head, code, base }
    }
    fn run(&mut self, input: i32) -> Option<i32> {
        let mut input = Some(input);
        loop {
            let op = parse_op(&self.code, self.head);
            let value = |p: Param| match p {
                Param::Pos(pos) => self.code[pos],
                Param::Im(im) => im,
                Param::Rel(rel) => {
                    let pos = (self.base as i32 + rel) as usize;
                    self.code[pos]
                }
            };
            match op {
                Op::Add([p1, p2, Param::Pos(pos)]) => {
                    self.code[pos] = value(p1) + value(p2);
                    self.head += 4;
                }
                Op::Mul([p1, p2, Param::Pos(pos)]) => {
                    self.code[pos] = value(p1) * value(p2);
                    self.head += 4;
                }
                Op::In(Param::Pos(pos)) => {
                    self.code[pos] = input.take()?;
                    self.head += 2;
                }
                Op::Out(param) => {
                    let out = value(param);
                    self.head += 2;
                    return Some(out);
                }
                Op::JumpTrue([p1, p2]) => {
                    if value(p1) != 0 {
                        self.head = value(p2) as usize;
                    } else {
                        self.head += 3;
                    }
                }
                Op::JumpFalse([p1, p2]) => {
                    if value(p1) == 0 {
                        self.head = value(p2) as usize;
                    } else {
                        self.head += 3;
                    }
                }
                Op::Less([p1, p2, Param::Pos(pos)]) => {
                    self.code[pos] = if value(p1) < value(p2) { 1 } else { 0 };
                    self.head += 4;
                }
                Op::Equal([p1, p2, Param::Pos(pos)]) => {
                    self.code[pos] = if value(p1) == value(p2) { 1 } else { 0 };
                    self.head += 4;
                }
                Op::OffsetBase(param) => {
                    let offset = value(param);
                    self.base = (self.base as i32 + offset) as usize;
                    self.head += 2;
                }
                Op::Halt => return None,
                _ => panic!("bad op"),
            }
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
        2 => Param::Rel(val),
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
        9 => Op::OffsetBase(make_param(code[head + 1], c)),
        bad => panic!("bad op code ({}) at ({})", bad, head),
    }
}
fn run_program_chain(
    mut input: i32,
    programs: &mut [&mut Program],
) -> Option<i32> {
    for p in programs {
        input = p.run(input)?;
    }
    Some(input)
}
#[cfg(test)]
mod tests {
    use super::run_program_chain;
    use super::Program;
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
            let mut prog_a = Program::new(0, amp_code.clone(), 0);
            let mut prog_b = Program::new(0, amp_code.clone(), 0);
            let mut prog_c = Program::new(0, amp_code.clone(), 0);
            let mut prog_d = Program::new(0, amp_code.clone(), 0);
            let mut prog_e = Program::new(0, amp_code.clone(), 0);
            prog_a.run(set[0]);
            prog_b.run(set[1]);
            prog_c.run(set[2]);
            prog_d.run(set[3]);
            prog_e.run(set[4]);
            let out_a = prog_a.run(0).unwrap();
            let out_b = prog_b.run(out_a).unwrap();
            let out_c = prog_c.run(out_b).unwrap();
            let out_d = prog_d.run(out_c).unwrap();
            let out_e = prog_e.run(out_d).unwrap();
            max_out = cmp::max(max_out, out_e);
        }
        assert_eq!(max_out, 914_828);
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
            let mut prog_a = Program::new(0, amp_code.clone(), 0);
            let mut prog_b = Program::new(0, amp_code.clone(), 0);
            let mut prog_c = Program::new(0, amp_code.clone(), 0);
            let mut prog_d = Program::new(0, amp_code.clone(), 0);
            let mut prog_e = Program::new(0, amp_code.clone(), 0);
            prog_a.run(set[0]);
            prog_b.run(set[1]);
            prog_c.run(set[2]);
            prog_d.run(set[3]);
            prog_e.run(set[4]);

            let mut input = 0;

            while let Some(output) = run_program_chain(
                input,
                &mut [
                    &mut prog_a,
                    &mut prog_b,
                    &mut prog_c,
                    &mut prog_d,
                    &mut prog_e,
                ],
            ) {
                input = output;
                max_out = cmp::max(output, max_out);
            }
        }
        assert_eq!(max_out, 17_956_613);
    }
}
