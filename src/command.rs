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

impl std::str::FromStr for CMMD {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        _ = s;
        todo!()
        // Once this is finished, remove the corresponding
        // `#[ignore = "not yet implemented"]` attributes below.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "not yet implemented"]
    fn parse_linear_cmmds() {
        assert_eq!(
            "LIN X5 Y0 Z0".parse(),
            Ok(CMMD::Linear(LinearCMMD {
                x: 5.0,
                y: 0.0,
                z: 0.0
            }))
        );
        assert_eq!(
            "LIN X0 Y5 Z0".parse(),
            Ok(CMMD::Linear(LinearCMMD {
                x: 0.0,
                y: 5.0,
                z: 0.0
            }))
        );
        assert_eq!(
            "LIN X0 Y0 Z5".parse(),
            Ok(CMMD::Linear(LinearCMMD {
                x: 0.0,
                y: 0.0,
                z: 5.0
            }))
        );
        assert_eq!(
            "LIN X5 Y5 Z5".parse(),
            Ok(CMMD::Linear(LinearCMMD {
                x: 5.0,
                y: 5.0,
                z: 5.0
            }))
        );
        assert_eq!(
            "LIN X0 Y0 Z0".parse(),
            Ok(CMMD::Linear(LinearCMMD {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }))
        );
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn parse_rotational_cmmds_cw() {
        assert_eq!(
            "CW X7.5 Y7.5 Z5 I1.25 J1.25 K5".parse(),
            Ok(CMMD::Rotational(RotationalCMMD {
                ccw: false,
                x: 7.5,
                y: 7.5,
                z: 5.0,
                i: 1.25,
                j: 1.25,
                k: 5.0,
            }))
        );
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn parse_rotational_cmmds_ccw() {
        assert_eq!(
            "CCW X5 Y5 Z5 I1.25 J1.25 K5".parse(),
            Ok(CMMD::Rotational(RotationalCMMD {
                ccw: true,
                x: 5.0,
                y: 5.0,
                z: 5.0,
                i: 1.25,
                j: 1.25,
                k: 5.0,
            }))
        );
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn parse_invalid_linear_cmmd() {
        let invalid_inputs = vec!["LIN X- Y0 Z0", "LIN 5 Y5 Z5", "LIX X5 Y5 Z5", "LINX5Y5Z5"];
        for input in invalid_inputs {
            assert!(input.parse::<CMMD>().is_err(), "Failed to catch invalid input: {}", input);
        }
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn parse_invalid_rotational_cmmd() {
        let invalid_inputs = vec![
            "CW X Y Z I J K",
            "CCW X7.5 Y7.5 Z I1.25 J1.25 K",
            "CCW 7.5 7.5 5 1.25 1.25 5",
            "CW X7.5Y7.5Z5I1.25J1.25K5",
        ];
        for input in invalid_inputs {
            assert!(input.parse::<CMMD>().is_err(), "Failed to catch invalid input: {}", input);
        }
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn parse_unrecognized_cmmd() {
        let invalid_inputs = vec!["XYZ X1 Y2 Z3", "ROT X0 Y0 Z0 I0 J0 K1"];
        for input in invalid_inputs {
            assert!(input.parse::<CMMD>().is_err(), "Failed to catch unrecognized command: {}", input);
        }
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn parse_empty_or_whitespace() {
        let invalid_inputs = vec!["", " ", "     "];
        for input in invalid_inputs {
            assert!(input.parse::<CMMD>().is_err(), "Failed to catch empty or whitespace input: '{}'", input);
        }
    }
}
