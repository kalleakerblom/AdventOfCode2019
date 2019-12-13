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
pub struct Program {
    head: usize,
    code: Vec<i64>,
    base: usize,
    input: Option<i64>,
}
impl Program {
    pub fn new(code: Vec<i64>) -> Self {
        Program { head: 0, code, base: 0, input: None }
    }
    pub fn run_input(&mut self, input: Option<i64>) -> Option<i64> {
        self.input = input;
        self.run()
    }
    pub fn set_input(&mut self, input: Option<i64>) {
        self.input = input;
    }
    pub fn run(&mut self) -> Option<i64> {
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
                    self.code[pos] = self.input.take()?;
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
