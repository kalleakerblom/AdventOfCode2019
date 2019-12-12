use cgmath::Vector3;
type Vector = Vector3<i32>;

#[derive(Debug, Clone, PartialEq)]
struct Moon {
    pos: Vector,
    vel: Vector,
}
impl Moon {
    fn new(pos: Vector) -> Self {
        Moon { vel: Vector::new(0, 0, 0), pos }
    }
    fn calculate_gravity(&mut self, other_moons: &[&Moon]) -> Vector {
        let mut gravity = Vector::new(0, 0, 0);
        for other in other_moons {
            let rel_pos = other.pos - self.pos;
            let norm = |val: i32| if val == 0 { 0 } else { val / val.abs() };
            let normalized_rel_pos = Vector3::new(
                norm(rel_pos[0]),
                norm(rel_pos[1]),
                norm(rel_pos[2]),
            );
            gravity += normalized_rel_pos;
        }
        gravity
    }
    fn move_self(&mut self) {
        self.pos += self.vel;
    }
    fn calculate_energy(&self) -> i32 {
        let pot_energy =
            self.pos[0].abs() + self.pos[1].abs() + self.pos[2].abs();
        let kin_energy =
            self.vel[0].abs() + self.vel[1].abs() + self.vel[2].abs();
        pot_energy * kin_energy
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Moon1D {
    pos: i32,
    vel: i32,
}
impl Moon1D {
    fn new(pos: i32) -> Self {
        Self { vel: 0, pos }
    }
    fn move_self(&mut self) {
        self.pos += self.vel;
    }
    fn calculate_gravity(&mut self, other_moons: &[&Moon1D]) -> i32 {
        let mut gravity = 0;
        for other in other_moons {
            gravity += if other.pos > self.pos {
                1
            } else if other.pos < self.pos {
                -1
            } else {
                0
            };
        }
        gravity
    }
}

#[cfg(test)]
mod tests {
    use super::Moon;
    use super::Moon1D;
    use super::Vector;
    #[test]
    fn day12_examp1e1() {
        let mut m0 = Moon::new(Vector::new(-1, 0, 2));
        let mut m1 = Moon::new(Vector::new(2, -10, -7));
        let mut m2 = Moon::new(Vector::new(4, -8, 8));
        let mut m3 = Moon::new(Vector::new(3, 5, -1));
        for _ in 0..10 {
            let g0 = m0.calculate_gravity(&[&m1, &m2, &m3]);
            let g1 = m1.calculate_gravity(&[&m0, &m2, &m3]);
            let g2 = m2.calculate_gravity(&[&m1, &m0, &m3]);
            let g3 = m3.calculate_gravity(&[&m1, &m2, &m0]);
            m0.vel += g0;
            m1.vel += g1;
            m2.vel += g2;
            m3.vel += g3;
            m0.move_self();
            m1.move_self();
            m2.move_self();
            m3.move_self();
        }
        assert_eq!(
            [m0, m1, m2, m3].iter().map(|m| m.calculate_energy()).sum::<i32>(),
            179
        );
    }
    #[test]
    fn day12_part1() {
        let mut m0 = Moon::new(Vector::new(-17, 9, -5));
        let mut m1 = Moon::new(Vector::new(-1, 7, 13));
        let mut m2 = Moon::new(Vector::new(-19, 12, 5));
        let mut m3 = Moon::new(Vector::new(-6, -6, -4));
        for _ in 0..1000 {
            let g0 = m0.calculate_gravity(&[&m1, &m2, &m3]);
            let g1 = m1.calculate_gravity(&[&m0, &m2, &m3]);
            let g2 = m2.calculate_gravity(&[&m1, &m0, &m3]);
            let g3 = m3.calculate_gravity(&[&m1, &m2, &m0]);
            m0.vel += g0;
            m1.vel += g1;
            m2.vel += g2;
            m3.vel += g3;
            m0.move_self();
            m1.move_self();
            m2.move_self();
            m3.move_self();
        }
        assert_eq!(
            [m0, m1, m2, m3].iter().map(|m| m.calculate_energy()).sum::<i32>(),
            8742
        );
    }

    #[test]
    fn day12_part2_1d() {
        //  (-17, 9, -5)
        //  (-1, 7, 13))
        //  (-19, 12, 5)
        //  (-6, -6, -4)
        // X period = 186028
        // y period = 231614
        // Z period = 60424
        // ans2 = lcm(x_period,y_period,z_period)
        let mut m0 = Moon1D::new(-5);
        let m0_ = m0.clone();
        let mut m1 = Moon1D::new(13);
        let m1_ = m1.clone();
        let mut m2 = Moon1D::new(5);
        let m2_ = m2.clone();
        let mut m3 = Moon1D::new(-4);
        let m3_ = m3.clone();

        for step in 1..300000 {
            let g0 = m0.calculate_gravity(&[&m1, &m2, &m3]);
            let g1 = m1.calculate_gravity(&[&m0, &m2, &m3]);
            let g2 = m2.calculate_gravity(&[&m1, &m0, &m3]);
            let g3 = m3.calculate_gravity(&[&m1, &m2, &m0]);
            m0.vel += g0;
            m1.vel += g1;
            m2.vel += g2;
            m3.vel += g3;
            m0.move_self();
            m1.move_self();
            m2.move_self();
            m3.move_self();
            if m0 == m0_ && m1 == m1_ && m2 == m2_ && m3 == m3_ {
                println!("step: {}", step);
                break;
            }
        }
    }
}
