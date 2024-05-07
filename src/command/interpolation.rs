use super::{Clockness, CmmdDestination, LinearCMMD, RotationalCMMD, CMMD};
use lina::{point2, ApproxEq as _, Point3, Radians, Vec3};
use std::{iter::FusedIterator, ops::Range};

const FIVE_DEGREES: Radians<f64> = Radians(0.08726646259971647);

/// The [`get_nth_point`](Interpolateable::get_nth_point) method gives us one of the points we need to trace the path
/// according to the project specification, assuming `start` is the destination of the last command.
///
/// The [`get_count`](Interpolateable::get_count) method gives us the amount of points found
/// between the start and destination points plus one to include either the start or the
/// destination points. It is necessary to call `get_count` before calling `get_nth_point`,
/// so as not to exceed its max input.
///
/// This is designed to be used to the same effect as an Iterator:
/// ```
/// use libproj2::command::{LinearCMMD, Interpolateable as _, CmmdDestination as _};
///
/// let last = LinearCMMD::new(0.0, 0.0, 0.0);
/// let start = last.get_destination();
///
/// let input = LinearCMMD::new(0.0, 5.0, 1.0);
///
/// assert_eq!(input.get_count(start), Ok(6));
///
/// for n in 1..=input.get_count(start).unwrap() {
///     println!("{:?}", input.get_nth_point(start, n));
/// }
/// ```
/// or:
/// ```
/// use libproj2::command::Interpolateable;
/// use libproj2::Point3;
///
/// pub fn interpolate<'a, T: Interpolateable>(
///     start: &'a Point3<f64>,
///     command: &'a T,
/// ) -> Result<impl Iterator<Item = Point3<f64>> + 'a, T::Error> {
///     let iter = (1..=command.get_count(*start)?).map(|n| command.get_nth_point(*start, n));
///     Ok(iter)
/// }
/// ```
/// However, this trait doesn't provide a way to cache intermediate results, so
/// calling its methods repeatedly may be less performant than using custom code.
pub trait Interpolateable: CmmdDestination + Clone {
    type Error: std::fmt::Debug;

    fn get_count(&self, start: Point3<f64>) -> Result<usize, Self::Error>;

    /// When `n` is zero, the function should yield `start`. When `n` is one, it should
    /// yield a point one increment to from `start` unless [`get_count()`](Interpolateable::get_count) is one. When `n`
    /// is `get_count()`, it should yield the destination point.
    ///
    /// The caller must ensure that `n` is no more than `get_count()`. If `debug_assertions`
    /// are enabled for this crate and `n` is greater than `get_count()`, the function
    /// *will* panic. For the same input when `debug_assertions` are enabled, the function
    /// *may* panic or yield garbage.
    ///
    /// It is also assumed that every field of `start` is finite. This shouldn't be a problem
    /// when `start` is the result of [`get_destination()`](super::CmmdDestination::get_destination).
    fn get_nth_point(&self, start: Point3<f64>, n: usize) -> Point3<f64>;
}

impl Interpolateable for LinearCMMD {
    type Error = std::convert::Infallible;

    /// This function will never return its error type, but it may panic.
    fn get_count(&self, start: Point3<f64>) -> Result<usize, Self::Error> {
        assert!(start.x.is_finite() && start.y.is_finite() && start.z.is_finite());
        Ok(std::cmp::max(
            1,
            (self.destination - start).length().ceil() as usize,
        ))
    }

    fn get_nth_point(&self, start: Point3<f64>, n: usize) -> Point3<f64> {
        debug_assert!(n <= self.get_count(start).unwrap());

        let total_vector = self.destination - start;
        if n as f64 * n as f64 >= total_vector.length2() {
            self.destination
        } else {
            start + total_vector.normalized() * n as _
        }
    }
}

impl Interpolateable for RotationalCMMD {
    type Error = &'static str;

    fn get_count(&self, start: Point3<f64>) -> Result<usize, Self::Error> {
        use lina::{atan2, Vec2};
        fn angle_past(dir: Clockness, point_a: Vec2<f64>, point_base: Vec2<f64>) -> Radians<f64> {
            let angle_a = atan2(point_a.y, point_a.x);
            let angle_base = atan2(point_base.y, point_base.x);

            let unnormalized_diff = (angle_a - angle_base) * dir.factor();

            unnormalized_diff.normalized()
        }

        let start2 = point2(start.x, start.y);
        let dest2 = point2(self.destination.x, self.destination.y);

        assert!(start2.x.is_finite() && start2.y.is_finite());

        if self.center == start2 {
            return Err("starting point indistinguishable from circle center");
        }

        let start_vector = start2 - self.center;
        let destination_vector = dest2 - self.center;

        if !(destination_vector
            .length()
            .approx_eq_abs(start_vector.length(), 0.0001)
            || destination_vector
                .length()
                .approx_eq_rel(start_vector.length(), f32::EPSILON.into()))
        {
            return Err("starting point different distance to circle center than destination");
        }

        let fuzzy_steps = angle_past(self.spin, destination_vector, start_vector) / FIVE_DEGREES;

        Ok(std::cmp::max(1, fuzzy_steps.ceil() as _))
    }

    fn get_nth_point(&self, start: Point3<f64>, n: usize) -> Point3<f64> {
        let count = self.get_count(start).unwrap();
        assert!(n <= count);

        if n == count {
            return self.destination;
        }

        let start_vec: Vec3<f64> = start - lina::point3(self.center.x, self.center.y, 0.0);
        let mut sdir: lina::SphericalDir<f64> = start_vec.into();
        assert_eq!(sdir.theta, Radians::quarter_turn());

        sdir.phi += FIVE_DEGREES * self.spin.factor() * n as f64;

        let base: Point3<f64> = lina::point3(self.center.x, self.center.y, self.destination.z);
        base + sdir.to_unit_vec()
    }
}

/// CMMD's implementation of this trait is just a wrapper around
/// [`LinearCMMD`]'s and [`RotationalCMMD`]'s implementation.
impl Interpolateable for CMMD {
    type Error = <RotationalCMMD as Interpolateable>::Error;

    fn get_count(&self, start: Point3<f64>) -> Result<usize, Self::Error> {
        match self {
            Self::Linear(x) => Ok(x.get_count(start).unwrap()),
            Self::Rotational(x) => x.get_count(start),
        }
    }

    fn get_nth_point(&self, start: Point3<f64>, n: usize) -> Point3<f64> {
        match self {
            Self::Linear(x) => x.get_nth_point(start, n),
            Self::Rotational(x) => x.get_nth_point(start, n),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct CmmdIter {
    cmmd: CMMD,
    start: Point3<f64>,
    range: Range<usize>,
}

impl CmmdIter {
    pub fn new(
        cmmd: CMMD,
        start: Point3<f64>,
    ) -> Result<Self, <RotationalCMMD as Interpolateable>::Error> {
        Ok(CmmdIter {
            cmmd,
            start,
            range: Range {
                start: 1,
                end: cmmd.get_count(start)?.checked_add(1).unwrap(),
            },
        })
    }
}

impl Iterator for CmmdIter {
    type Item = Point3<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.range
            .next()
            .map(|n| self.cmmd.get_nth_point(self.start, n))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.range
            .nth(n)
            .map(|x| self.cmmd.get_nth_point(self.start, x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl DoubleEndedIterator for CmmdIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range
            .next_back()
            .map(|x| self.cmmd.get_nth_point(self.start, x))
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.range
            .nth_back(n)
            .map(|x| self.cmmd.get_nth_point(self.start, x))
    }
}

impl ExactSizeIterator for CmmdIter {
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl FusedIterator for CmmdIter {}
