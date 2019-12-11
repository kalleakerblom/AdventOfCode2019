use super::int_code::Program;

fn run_program_chain(
    mut input: i64,
    programs: &mut [&mut Program],
) -> Option<i64> {
    for p in programs {
        input = p.run(Some(input))?;
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
        let amp_code: Vec<i64> = fs::read_to_string("input/day7")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let mut max_out = 0;
        for set in (0..5).permutations(5) {
            let mut prog_a = Program::new(amp_code.clone());
            let mut prog_b = Program::new(amp_code.clone());
            let mut prog_c = Program::new(amp_code.clone());
            let mut prog_d = Program::new(amp_code.clone());
            let mut prog_e = Program::new(amp_code.clone());
            prog_a.run(Some(set[0]));
            prog_b.run(Some(set[1]));
            prog_c.run(Some(set[2]));
            prog_d.run(Some(set[3]));
            prog_e.run(Some(set[4]));
            let out_a = prog_a.run(Some(0)).unwrap();
            let out_b = prog_b.run(Some(out_a)).unwrap();
            let out_c = prog_c.run(Some(out_b)).unwrap();
            let out_d = prog_d.run(Some(out_c)).unwrap();
            let out_e = prog_e.run(Some(out_d)).unwrap();
            max_out = cmp::max(max_out, out_e);
        }
        assert_eq!(max_out, 914_828);
    }
    #[test]
    fn day7_part2() {
        let amp_code: Vec<i64> = fs::read_to_string("input/day7")
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut max_out = 0;

        for set in (5..10).permutations(5) {
            let mut prog_a = Program::new(amp_code.clone());
            let mut prog_b = Program::new(amp_code.clone());
            let mut prog_c = Program::new(amp_code.clone());
            let mut prog_d = Program::new(amp_code.clone());
            let mut prog_e = Program::new(amp_code.clone());
            prog_a.run(Some(set[0]));
            prog_b.run(Some(set[1]));
            prog_c.run(Some(set[2]));
            prog_d.run(Some(set[3]));
            prog_e.run(Some(set[4]));

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
