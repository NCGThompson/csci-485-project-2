pub mod interpolation;
#[cfg(test)]
mod tests;

use lina::{point3, Point3};

pub use self::interpolation::Interpolateable;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CMMD {
    Linear(LinearCMMD),
    Rotational(RotationalCMMD),
}

impl std::str::FromStr for CMMD {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
        // Once this is finished, remove the corresponding
        // `#[ignore = "not yet implemented"]` attributes below.
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct LinearCMMD {
    destination: Point3<f64>,
}

impl LinearCMMD {
    pub fn new(x: f64, y: f64, z: f64) -> LinearCMMD {
        assert!(x.is_finite() && y.is_finite() && z.is_finite());

        LinearCMMD {
            destination: point3(x, y, z),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct RotationalCMMD {
    ccw: bool,
    destination: Point3<f64>,
    center: Point3<f64>,
}

impl RotationalCMMD {
    pub fn new(spin: bool, x: f64, y: f64, z: f64, i: f64, j: f64, k: f64) -> RotationalCMMD {
        assert!(
            x.is_finite()
                && y.is_finite()
                && z.is_finite()
                && i.is_finite()
                && j.is_finite()
                && k.is_finite()
        );

        RotationalCMMD {
            ccw: spin,
            destination: point3(x, y, z),
            center: point3(i, j, k),
        }
    }
}

/// Every input command type explicitly states the coordinates where the walker should end up.
/// This is a generic getter for those coordinates.
pub trait CmmdDestination {
    fn get_destination(&self) -> Point3<f64>;
}

impl CmmdDestination for CMMD {
    /// Returns the `destination` attribute for either of CMMD's inner structs.
    ///
    /// Example
    ///
    /// ```
    /// use libproj2::command::{CmmdDestination as _, LinearCMMD, RotationalCMMD, CMMD};
    /// use lina::point3;
    ///
    /// let cmmd1 = CMMD::Linear(LinearCMMD::new(1.0, 2.0, 3.0));
    /// let cmmd2 = CMMD::Rotational(RotationalCMMD::new(false, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0));
    ///
    /// let destination = cmmd1.get_destination();
    /// assert_eq!(destination, point3(1.0, 2.0, 3.0));
    /// assert_eq!(destination, match cmmd1 {
    ///     CMMD::Linear(inner) => inner.get_destination(),
    ///     _ => unreachable!(),
    /// });
    /// assert_eq!(destination, cmmd2.get_destination());
    /// ```
    #[inline]
    fn get_destination(&self) -> Point3<f64> {
        match self {
            Self::Linear(x) => x.get_destination(),
            Self::Rotational(x) => x.get_destination(),
        }
    }
}

impl CmmdDestination for LinearCMMD {
    #[inline(always)]
    fn get_destination(&self) -> Point3<f64> {
        self.destination
    }
}

impl CmmdDestination for RotationalCMMD {
    #[inline(always)]
    fn get_destination(&self) -> Point3<f64> {
        self.destination
    }
}
