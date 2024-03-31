mod parseing {
    use super::super::*;

    #[test]
    #[ignore = "not yet implemented"]
    fn parse_linear_cmmds() {
        assert_eq!(
            "LIN X5 Y0 Z0".parse(),
            Ok(CMMD::Linear(LinearCMMD {
                destination: point3(5.0, 0.0, 0.0),
            }))
        );
        assert_eq!(
            "LIN X0 Y5 Z0".parse(),
            Ok(CMMD::Linear(LinearCMMD {
                destination: point3(0.0, 5.0, 0.0),
            }))
        );
        assert_eq!(
            "LIN X0 Y0 Z5".parse(),
            Ok(CMMD::Linear(LinearCMMD {
                destination: point3(0.0, 0.0, 5.0),
            }))
        );
        assert_eq!(
            "LIN X5 Y5 Z5".parse(),
            Ok(CMMD::Linear(LinearCMMD {
                destination: point3(5.0, 5.0, 5.0),
            }))
        );
        assert_eq!(
            "LIN X0 Y0 Z0".parse(),
            Ok(CMMD::Linear(LinearCMMD {
                destination: point3(0.0, 0.0, 0.0),
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
                destination: point3(7.5, 7.5, 5.0),
                center: point3(1.25, 1.25, 5.0),
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
                destination: point3(5.0, 5.0, 5.0),
                center: point3(1.25, 1.25, 5.0),
            }))
        );
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn parse_invalid_linear_cmmd() {
        let invalid_inputs = vec!["LIN X- Y0 Z0", "LIN 5 Y5 Z5", "LIX X5 Y5 Z5", "LINX5Y5Z5"];
        for input in invalid_inputs {
            assert!(
                input.parse::<CMMD>().is_err(),
                "Failed to catch invalid input: {}",
                input
            );
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
            assert!(
                input.parse::<CMMD>().is_err(),
                "Failed to catch invalid input: {}",
                input
            );
        }
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn parse_unrecognized_cmmd() {
        let invalid_inputs = vec!["XYZ X1 Y2 Z3", "ROT X0 Y0 Z0 I0 J0 K1"];
        for input in invalid_inputs {
            assert!(
                input.parse::<CMMD>().is_err(),
                "Failed to catch unrecognized command: {}",
                input
            );
        }
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn parse_empty_or_whitespace() {
        let invalid_inputs = vec!["", " ", "     "];
        for input in invalid_inputs {
            assert!(
                input.parse::<CMMD>().is_err(),
                "Failed to catch empty or whitespace input: '{}'",
                input
            );
        }
    }
}
