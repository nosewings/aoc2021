use combine::Parser;
use itertools::Itertools;

#[derive(Clone, Copy)]
pub struct Input {
    x_lo: i32,
    x_hi: i32,
    y_lo: i32,
    y_hi: i32,
}

impl Input {
    fn in_target(self, x: i32, y: i32) -> bool {
        x >= self.x_lo && x <= self.x_hi && y >= self.y_lo && y <= self.y_hi
    }

    fn min_vx(self) -> i32 {
        assert!(self.x_lo >= 0);
        let x_lo = self.x_lo as f32;
        // The minimum feasible vx₀ makes us just barely graze the
        // left side of the target x-region before drag causes vx to
        // become 0.  The final x-coordinate for a given vx₀ is
        //
        // vx₀ + (vx₀ - 1) + ⋯ + 1 = vx₀(vx₀ + 1)/2.
        //
        // Solving the equation vx₀(vx₀ + 1)/2 = x for vx₀ yields the
        // (positive) solution vx₀ = (√(8x + 1) - 1)/2.
        let vx = ((8.0 * x_lo + 1.0).sqrt() - 1.0) / 2.0;
        vx.ceil() as i32
    }

    fn max_vx(self) -> i32 {
        assert!(self.x_hi >= 0);
        self.x_hi
    }

    fn min_vy(self) -> i32 {
        assert!(self.y_lo <= 0);
        self.y_lo
    }

    fn max_vy(self) -> i32 {
        assert!(self.y_lo <= 0);
        // The maximum feasible vy₀ makes us just barely hit the
        // bottom of the target y-region on the way down (rather than
        // simply falling past it).  Note that y-coordinates are
        // symmetrical on the way up and the way down.
        -self.y_lo - 1
    }

    fn x_orbit(vx: i32) -> impl Iterator<Item = i32> {
        itertools::iterate((0, vx), |(x, vx)| (x + vx, (vx - 1).max(0))).map(|(x, _)| x)
    }

    fn y_orbit(vy: i32) -> impl Iterator<Item = i32> {
        itertools::iterate((0, vy), |(y, vy)| (y + vy, (vy - 1))).map(|(y, _)| y)
    }

    fn orbit(vx: i32, vy: i32) -> impl Iterator<Item = (i32, i32)> {
        Input::x_orbit(vx).zip(Input::y_orbit(vy))
    }

    fn is_valid(self, vx: i32, vy: i32) -> bool {
        Input::orbit(vx, vy)
            .take_while(|(x, y)| *x <= self.x_hi && *y >= self.y_lo)
            .any(|(x, y)| self.in_target(x, y))
    }

    pub fn max_apogee(self) -> i32 {
        for vy in (self.min_vy()..=self.max_vy()).rev() {
            for vx in self.min_vx()..=self.max_vx() {
                if self.is_valid(vx, vy) {
                    return vy * (vy + 1) / 2;
                }
            }
        }
        panic!("no solution")
    }

    pub fn valid_velocities(self) -> impl Iterator<Item = (i32, i32)> {
        (self.min_vx()..=self.max_vx())
            .cartesian_product(self.min_vy()..=self.max_vy())
            .filter(move |(vx, vy)| self.is_valid(*vx, *vy))
    }
}

pub fn parse_input<'a>() -> impl Parser<&'a str, Output = Input> {
    use combine::parser::char::*;
    use combine::*;

    use crate::combine_parse_integral;

    fn range<'a>(label: char) -> impl Parser<&'a str, Output = (i32, i32)> {
        char(label)
            .with(char('='))
            .with(combine_parse_integral())
            .skip(string(".."))
            .and(combine_parse_integral())
    }

    string("target area: ")
        .with(range('x'))
        .skip(string(", "))
        .and(range('y'))
        .skip(newline())
        .skip(eof())
        .map(|((x_lo, x_hi), (y_lo, y_hi))| Input {
            x_lo,
            x_hi,
            y_lo,
            y_hi,
        })
}
