use std::collections::HashSet;
use std::hash::Hash;
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

impl Direction {
    #[inline]
    #[allow(dead_code)]
    fn orthogonal(self, other: Self) -> bool {
        use self::Direction::*;
        match (self, other) {
            (Forward, Forward) |
            (Forward, Backward) |
            (Backward, Forward) |
            (Backward, Backward) => false,
            _ => true,
        }
    }

    #[inline]
    #[allow(dead_code)]
    fn perpendicular(self, other: Self) -> bool {
        !self.orthogonal(other)
    }
}

pub fn high_byte(b: u32) -> u32 {
    b >> 8 as u8 as u32
}

pub fn low_byte(b: u32) -> u32 {
    b as u8 as u32
}

pub trait Pop {
    type Item;
    fn pop(&mut self) -> Option<Self::Item>;
}

impl<T: Eq + Hash + Clone> Pop for HashSet<T> {
    type Item = T;
    fn pop(&mut self) -> Option<Self::Item> {
        self.iter().next().map(|cell| cell.clone()).map(|cell| {
            self.take(&cell).unwrap()
        })
    }
}

pub trait ConvCoord {
    fn axis_angle_z(&self) -> ::na::Vector3<f32>;
}

impl ConvCoord for ::na::Vector2<isize> {
    fn axis_angle_z(&self) -> ::na::Vector3<f32> {
        ::na::Vector3::new(self[0], self[1], 0).axis_angle_z()
    }
}

impl ConvCoord for ::na::Vector3<isize> {
    fn axis_angle_z(&self) -> ::na::Vector3<f32> {
        if *self == ::na::Vector3::new(-1, 0, 0) {
            ::na::Vector3::new(0.0, -FRAC_PI_2, 0.0)
        } else if *self == ::na::Vector3::new(1, 0, 0) {
            ::na::Vector3::new(0.0, FRAC_PI_2, 0.0)
        } else if *self == ::na::Vector3::new(0, -1, 0) {
            ::na::Vector3::new(FRAC_PI_2, 0.0, 0.0)
        } else if *self == ::na::Vector3::new(0, 1, 0) {
            ::na::Vector3::new(-FRAC_PI_2, 0.0, 0.0)
        } else if *self == ::na::Vector3::new(0, 0, -1) {
            ::na::Vector3::new(PI, 0.0, 0.0)
        } else if *self == ::na::Vector3::new(0, 0, 1) {
            ::na::Vector3::new(0.0, 0.0, 0.0)
        } else {
            panic!("invalid direction");
        }
    }
}
