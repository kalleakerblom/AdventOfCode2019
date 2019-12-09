#[derive(Debug)]
enum Param {
    Pos(usize),
    Rel(i64),
    Im(i64),
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
    code: Vec<i64>,
    base: usize,
}
impl Program {
    fn new(code: Vec<i64>) -> Self {
        Program { head: 0, code, base: 0 }
    }
    fn run(&mut self, input: i64) -> Option<i64> {
        let mut input = Some(input);
        loop {
            let op = parse_op(&self.code, self.head);
            let value = |p: Param| match p {
                Param::Pos(pos) => self.code[pos],
                Param::Im(im) => im,
                Param::Rel(rel) => {
                    let pos = (self.base as i64 + rel) as usize;
                    self.code[pos]
                }
            };
            let pos = |p: Param| match p {
                Param::Pos(pos) => pos,
                Param::Rel(rel) => (self.base as i64 + rel) as usize,
                Param::Im(_) => panic!("immediate value invalid as pos"),
            };
            match op {
                Op::Add([p1, p2, p3]) => {
                    let pos = pos(p3);
                    self.code[pos] = value(p1) + value(p2);
                    self.head += 4;
                }
                Op::Mul([p1, p2, p3]) => {
                    let pos = pos(p3);
                    self.code[pos] = value(p1) * value(p2);
                    self.head += 4;
                }
                Op::In(param) => {
                    let pos = pos(param);
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
                Op::Less([p1, p2, p3]) => {
                    let pos = pos(p3);
                    self.code[pos] = if value(p1) < value(p2) { 1 } else { 0 };
                    self.head += 4;
                }
                Op::Equal([p1, p2, p3]) => {
                    let pos = pos(p3);
                    self.code[pos] = if value(p1) == value(p2) { 1 } else { 0 };
                    self.head += 4;
                }
                Op::OffsetBase(param) => {
                    let offset = value(param);
                    self.base = (self.base as i64 + offset) as usize;
                    self.head += 2;
                }
                Op::Halt => return None,
            }
        }
    }
}

fn parse_op(code: &[i64], head: usize) -> Op {
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

#[cfg(test)]
mod tests {
    use super::Program;
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
        while let Some(out) = prog.run(0) {
            println!("{:?}", out)
        }

        let mut code: Vec<i64> = "1102,34915192,34915192,7,4,7,99,0"
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        code.extend(iter::repeat(0).take(100));
        let mut prog = Program::new(code);
        assert_eq!(prog.run(0), Some(1_219_070_632_396_864));

        let mut code: Vec<i64> = "104,1125899906842624,99"
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        code.extend(iter::repeat(0).take(100));
        let mut prog = Program::new(code);
        assert_eq!(prog.run(0), Some(1_125_899_906_842_624));
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
        assert_eq!(prog.run(1), Some(3_742_852_857));
        // part 2
        let mut prog_2 = Program::new(code);
        assert_eq!(prog_2.run(2), Some(73439));
    }
}
