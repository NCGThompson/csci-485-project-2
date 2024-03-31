use lina::Point3;
use std::cmp::min;

use super::{CmmdDestination as _, LinearCMMD, RotationalCMMD, CMMD};

pub trait Interpolateable {
    fn get_count(&self, start: Point3<f64>) -> usize;

    fn get_nth_point(&self, start: Point3<f64>, n: usize) -> Point3<f64>;
}

impl Interpolateable for LinearCMMD {
    fn get_count(&self, start: Point3<f64>) -> usize {
        min(1, (self.get_destination() - start).length().ceil() as usize)
    }

    fn get_nth_point(&self, start: Point3<f64>, n: usize) -> Point3<f64> {
        assert!(n <= self.get_count(start));
        todo!();
    }
}

impl Interpolateable for RotationalCMMD {
    fn get_count(&self, start: Point3<f64>) -> usize {
        let _ = start;
        todo!()
    }

    fn get_nth_point(&self, start: Point3<f64>, n: usize) -> Point3<f64> {
        let _ = start;
        let _ = n;
        todo!()
    }
}

impl Interpolateable for CMMD {
    fn get_count(&self, start: Point3<f64>) -> usize {
        let _ = start;
        todo!()
    }

    fn get_nth_point(&self, start: Point3<f64>, n: usize) -> Point3<f64> {
        let _ = start;
        let _ = n;
        todo!()
    }
}
