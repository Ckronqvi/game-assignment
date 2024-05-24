use std::fmt::{Display, Formatter, Result};

use crate::point::Point2d;
use crate::traits::{Draw, Position};
use crate::unit::Player;

pub struct Hud<'a> {
    score: u32,
    player: &'a Player,
    y_position: u16,
    position: Point2d<u16>,
}

impl<'a> Hud<'a> {
    pub fn new(score: u32, player: &'a Player, y_position: u16) -> Self {
        Hud {
            score,
            player,
            y_position,
            position: Point2d {
                x: 0,
                y: y_position,
            },
        }
    }
}

impl Display for Hud<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Score:{} \n Health:{} \n Speed{:.3}",
            self.score,
            self.player.health(),
            self.player.speed()
        )
    }
}

impl Position<u16> for Hud<'_> {
    fn position(&self) -> Point2d<u16> {
        self.position
    }

    fn set_position(&mut self, position: Point2d<u16>) {
        self.position = position;
    }
}

impl Draw<u16> for Hud<'_> {}
