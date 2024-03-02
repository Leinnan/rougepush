pub mod utils;

use bevy::prelude::Reflect;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, PartialEq, Eq, Hash, Reflect)]
pub struct Vector2Int {
    pub x: i32,
    pub y: i32,
}
impl From<(i32, i32)> for Vector2Int {
    fn from(tuple: (i32, i32)) -> Self {
        Vector2Int {
            x: tuple.0,
            y: tuple.1,
        }
    }
}
impl Vector2Int {
    pub const UP: Vector2Int = Vector2Int { x: 0, y: 1 };
    pub const DOWN: Vector2Int = Vector2Int { x: 0, y: -1 };
    pub const LEFT: Vector2Int = Vector2Int { x: -1, y: 0 };
    pub const RIGHT: Vector2Int = Vector2Int { x: 1, y: 0 };
    pub fn new(x: i32, y: i32) -> Vector2Int {
        Vector2Int { x, y }
    }
    pub fn manhattan(&self, other: Vector2Int) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Vector2Int {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2Int::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vector2Int {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Vector2Int {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2Int::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vector2Int {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Div<i32> for Vector2Int {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        Vector2Int::new(self.x / other, self.y / other)
    }
}

impl Mul<i32> for Vector2Int {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Vector2Int::new(self.x * other, self.y * other)
    }
}

impl Mul<Vector2Int> for i32 {
    type Output = Vector2Int;

    fn mul(self, other: Vector2Int) -> Vector2Int {
        Vector2Int::new(other.x * self, other.y * self)
    }
}

#[allow(dead_code)]
pub const ORTHO_DIRECTIONS: [Vector2Int; 4] = [
    Vector2Int::UP,
    Vector2Int::DOWN,
    Vector2Int::LEFT,
    Vector2Int::RIGHT,
];
