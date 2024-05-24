use num::{traits::NumAssign, NumCast};
use rand::{
    distributions::{uniform::SampleUniform, Standard},
    prelude::Distribution,
    rngs::ThreadRng,
    Rng,
};
use std::ops::Range;
use std::{fmt::Display, io::Write};

use crate::point::Point2d;

pub trait Position<T: NumAssign + Copy> {
    fn position(&self) -> Point2d<T>;
    fn set_position(&mut self, position: Point2d<T>);
    fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<T>, y_range: Range<T>)
    where
        T: PartialOrd + SampleUniform,
        Standard: Distribution<T>,
    {
        let new_position = Point2d::new(rng.gen_range(x_range), rng.gen_range(y_range));
        self.set_position(new_position);
    }
}

pub trait Draw<T: NumAssign + Copy + NumCast>: Position<T> + Display {
    fn draw(&self, buffer: &mut impl Write) {
        let position = self.position();

        crossterm::queue!(
            buffer,
            crossterm::cursor::MoveTo(
                position
                    .x
                    .to_f64()
                    .expect("could not convert position x to f64")
                    .round() as u16
                    + 1,
                position
                    .y
                    .to_f64()
                    .expect("could not convert position y to f64")
                    .round() as u16
                    + 1,
            ),
            crossterm::style::Print(self)
        )
        .unwrap();
    }
}
