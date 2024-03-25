// For info on enums, run `rustup doc --book` and navigate to chapter 6.

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CMMD {
    Linear(LinearCMMD),
    Rotational(RotationalCMMD),
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct LinearCMMD {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct RotationalCMMD {
    ccw: bool,
    x: f64,
    y: f64,
    z: f64,
    i: f64,
    j: f64,
    k: f64,
}
