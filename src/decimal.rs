use crate::util::round_float;

/// Struct that represents prettified byte values (base-10)
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrettyBytes {
    num: f64,
    suffix: ByteValues,
}

impl std::fmt::Display for PrettyBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.num, self.suffix)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum ByteValues {
    B,
    KB,
    MB,
    GB,
    TB,
    PB,
    EB,
    ZB,
    YB,
}

impl ByteValues {
    const UNITS: [Self; 9] = [
        Self::B,
        Self::KB,
        Self::MB,
        Self::GB,
        Self::TB,
        Self::PB,
        Self::EB,
        Self::ZB,
        Self::YB,
    ];
}

/// Convert a byte value to a "prettified" version
///
/// Converts using base-10 byte suffixes (KB, MB, GB)
///
/// ## Example
/// ```
/// # use pretty_bytes_enum::pretty_bytes;
///
/// // No rounding
/// let prettified = pretty_bytes(1_000_000., None);
///
/// // Round to 2 decimal places
/// let prettified = pretty_bytes(3_564_234., Some(2));
/// ```
#[must_use]
#[allow(
    // Values are too small for truncation or wrap
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    // Sign is already checked and converted to positive
    clippy::cast_sign_loss
)]
pub fn pretty_bytes(num: f64, round_places: Option<u8>) -> PrettyBytes {
    let num = num.floor();
    let is_negative = num.is_sign_negative();
    let mut num = num.abs();

    let exponent = std::cmp::min(
        (num.log10() / 3.).floor() as usize,
        ByteValues::UNITS.len() - 1,
    );

    num /= 1000_f64.powi(exponent as i32);

    if let Some(round_places) = round_places {
        num = round_float(num, round_places);
    }

    let unit = ByteValues::UNITS[exponent];

    if is_negative {
        num = -num;
    }

    PrettyBytes { num, suffix: unit }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_pretty_bytes() {
        // Test '0'
        assert_eq!(
            pretty_bytes(0., None),
            PrettyBytes {
                num: 0.,
                suffix: ByteValues::B,
            }
        );

        // Test actual decimal values
        // (should always round down)
        assert_eq!(
            pretty_bytes(5.323, None),
            PrettyBytes {
                num: 5.,
                suffix: ByteValues::B,
            }
        );

        assert_eq!(
            pretty_bytes(5_430.999_999_999, None),
            PrettyBytes {
                num: 5.43,
                suffix: ByteValues::KB,
            }
        );

        // Test all unit values
        assert_eq!(
            pretty_bytes(1000_f64.powi(0), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::B,
            }
        );

        assert_eq!(
            pretty_bytes(1000_f64.powi(1), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::KB,
            }
        );

        assert_eq!(
            pretty_bytes(1000_f64.powi(2), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::MB,
            }
        );

        assert_eq!(
            pretty_bytes(1000_f64.powi(3), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::GB,
            }
        );

        assert_eq!(
            pretty_bytes(1000_f64.powi(4), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::TB,
            }
        );

        assert_eq!(
            pretty_bytes(1000_f64.powi(5), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::PB,
            }
        );

        assert_eq!(
            pretty_bytes(1000_f64.powi(6), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::EB,
            }
        );

        assert_eq!(
            pretty_bytes(1000_f64.powi(7), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::ZB,
            }
        );

        assert_eq!(
            pretty_bytes(1000_f64.powi(8), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::YB,
            }
        );

        // Test extra large values
        assert_eq!(
            pretty_bytes(35_000_000_000_000_000_000_000_000_000., None),
            PrettyBytes {
                num: 35000.,
                suffix: ByteValues::YB,
            }
        );

        // Various other tests
        assert_eq!(
            pretty_bytes(50060., None),
            PrettyBytes {
                num: 50.06,
                suffix: ByteValues::KB,
            }
        );

        assert_eq!(
            pretty_bytes(736_532_432., None),
            PrettyBytes {
                num: 736.532_432,
                suffix: ByteValues::MB,
            }
        );

        // Test rounding
        assert_eq!(
            pretty_bytes(5003., Some(2)),
            PrettyBytes {
                num: 5.,
                suffix: ByteValues::KB,
            }
        );

        assert_eq!(
            pretty_bytes(8_452_020., Some(2)),
            PrettyBytes {
                num: 8.45,
                suffix: ByteValues::MB,
            }
        );

        assert_eq!(
            pretty_bytes(55_700., Some(0)),
            PrettyBytes {
                num: 56.,
                suffix: ByteValues::KB,
            }
        );
    }
}
