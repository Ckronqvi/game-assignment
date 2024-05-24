use position_derive::Position;

use crate::{
    point::Point2d,
    traits::{Draw, Position},
};

#[derive(Debug, Default, Position)]
pub struct Collectible {
    position: Point2d<u16>,
}
impl Draw<u16> for Collectible {}
