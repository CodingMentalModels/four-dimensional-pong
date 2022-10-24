use bevy::prelude::*;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rotation {
    from: Axis4,
    to: Axis4,
    quarter_turns: usize,
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            from: Axis4::X,
            to: Axis4::X,
            quarter_turns: 0,
        }
    }
}

impl Rotation {

    pub fn new(from: Axis4, to: Axis4, quarter_turns: usize) -> Self {
        Self {
            from,
            to,
            quarter_turns: quarter_turns % 4,
        }
    }

    pub fn rotate(&self, v: Vec4) -> Vec4 {
        let mut v = v;
        for _ in 0..self.quarter_turns {
            v = self.rotate_quarter_turn(v);
        }
        return v;
    }

    fn rotate_quarter_turn(&self, v: Vec4) -> Vec4 {
        let mut to_return = v;
        let from_index = self.from.get_index();
        let to_index = self.to.get_index();

        to_return[from_index] = -v[to_index];
        to_return[to_index] = v[from_index];

        return to_return;
    }

}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Axis4 {
    X,
    Y,
    Z,
    W,
}

impl Axis4 {

    pub fn get_index(&self) -> usize {
        match self {
            Axis4::X => 0,
            Axis4::Y => 1,
            Axis4::Z => 2,
            Axis4::W => 3,
        }
    }
}

#[cfg(test)]
mod test_rotations {
    use super::*;

    #[test]
    fn test_rotations_rotate() {
        
        let zero = Vec4::ZERO;
        let i = Vec4::X;
        let j = Vec4::Y;
        let k = Vec4::Z;
        let h = Vec4::W;
        let one = Vec4::ONE;
        let complicated = Vec4::new(1., 2., 3., 4.);

        let trivial_rotation = Rotation::new(Axis4::X, Axis4::X, 0);
        assert_eq!(trivial_rotation.rotate(zero), zero);
        assert_eq!(trivial_rotation.rotate(one), one);
        
        let redundant_rotation = Rotation::new(Axis4::X, Axis4::X, 200);
        assert_eq!(trivial_rotation, redundant_rotation);

        let r_x_y = Rotation::new(Axis4::X, Axis4::Y, 1);
        assert_eq!(r_x_y.rotate(zero), zero);
        assert_eq!(r_x_y.rotate(i), j);
        assert_eq!(r_x_y.rotate(j), -i);
        assert_eq!(r_x_y.rotate(k), k);
        assert_eq!(r_x_y.rotate(h), h);
        assert_eq!(r_x_y.rotate(one), Vec4::new(-1., 1., 1., 1.));
        assert_eq!(r_x_y.rotate(complicated), Vec4::new(-2., 1., 3., 4.));

        
        let r_y_w = Rotation::new(Axis4::Y, Axis4::W, 1);
        assert_eq!(r_y_w.rotate(zero), zero);
        assert_eq!(r_y_w.rotate(i), i);
        assert_eq!(r_y_w.rotate(j), h);
        assert_eq!(r_y_w.rotate(k), k);
        assert_eq!(r_y_w.rotate(h), -j);
        assert_eq!(r_y_w.rotate(one), Vec4::new(1., -1., 1., 1.));
        assert_eq!(r_y_w.rotate(complicated), Vec4::new(1., -4., 3., 2.));

        
        let r_y_w = Rotation::new(Axis4::Y, Axis4::W, 2);
        assert_eq!(r_y_w.rotate(zero), zero);
        assert_eq!(r_y_w.rotate(i), i);
        assert_eq!(r_y_w.rotate(j), -j);
        assert_eq!(r_y_w.rotate(k), k);
        assert_eq!(r_y_w.rotate(h), -h);
        assert_eq!(r_y_w.rotate(one), Vec4::new(1.0, -1.0, 1.0, -1.0));
        assert_eq!(r_y_w.rotate(complicated), Vec4::new(1.0, -2.0, 3.0, -4.0));
        
    }
}