#![allow(clippy::module_name_repetitions)]

use crate::util::round_float;

/// Struct that represents prettified byte values (base-2)
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrettyBytesBinary {
    num: f64,
    suffix: ByteValuesBinary,
}

impl std::fmt::Display for PrettyBytesBinary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.num, self.suffix)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum ByteValuesBinary {
    B,
    KiB,
    MiB,
    GiB,
    TiB,
    PiB,
    EiB,
    ZiB,
    YiB,
}

impl ByteValuesBinary {
    const UNITS: [Self; 9] = [
        Self::B,
        Self::KiB,
        Self::MiB,
        Self::GiB,
        Self::TiB,
        Self::PiB,
        Self::EiB,
        Self::ZiB,
        Self::YiB,
    ];
}

/// Convert a byte value to a "prettified" version
///
/// Converts using base-2 byte suffixes (KiB, MiB, GiB)
///
/// ## Example
/// ```
/// # use pretty_bytes_enum::pretty_bytes_binary;
///
/// // No rounding
/// let prettified = pretty_bytes_binary(1_048_576., None);
/// assert_eq!(prettified.to_string(), "1 MiB");
///
/// // Round to 2 decimal places
/// let prettified = pretty_bytes_binary(3_195_498., Some(2));
/// assert_eq!(prettified.to_string(), "3.05 MiB");
/// ```
#[must_use]
pub fn pretty_bytes_binary(num: f64, round_places: Option<u8>) -> PrettyBytesBinary {
    let num = num.floor();
    let is_negative = num.is_sign_negative();
    let mut num = num.abs();

    let exponent = std::cmp::min(
        num.log(1024.).floor() as usize,
        ByteValuesBinary::UNITS.len() - 1,
    );

    num /= 1024_f64.powi(exponent as i32);

    if let Some(round_places) = round_places {
        num = round_float(num, round_places);
    }

    let unit = ByteValuesBinary::UNITS[exponent];

    if is_negative {
        num = -num;
    }

    PrettyBytesBinary { num, suffix: unit }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_pretty_bytes_binary() {
        // Test '0'
        assert_eq!(
            pretty_bytes_binary(0., None),
            PrettyBytesBinary {
                num: 0.,
                suffix: ByteValuesBinary::B,
            }
        );

        // Test actual decimal values
        // (should always round down)
        assert_eq!(
            pretty_bytes_binary(5.323, None),
            PrettyBytesBinary {
                num: 5.,
                suffix: ByteValuesBinary::B,
            }
        );

        // Test all unit values
        assert_eq!(
            pretty_bytes_binary(1024_f64.powi(0), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::B,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_f64.powi(1), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::KiB,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_f64.powi(2), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::MiB,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_f64.powi(3), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::GiB,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_f64.powi(4), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::TiB,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_f64.powi(5), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::PiB,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_f64.powi(6), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::EiB,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_f64.powi(7), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::ZiB,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_f64.powi(8), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::YiB,
            }
        );

        // Test extra large values
        assert_eq!(
            pretty_bytes_binary(1024_f64.powi(10), None),
            PrettyBytesBinary {
                num: 1024_f64.powi(2),
                suffix: ByteValuesBinary::YiB,
            }
        );

        // Test rounding
        assert_eq!(
            pretty_bytes_binary(5014., Some(2)),
            PrettyBytesBinary {
                num: 4.9,
                suffix: ByteValuesBinary::KiB,
            }
        );
    }
}
