#![allow(clippy::module_name_repetitions)]

use crate::util::round_float;

/// Struct that represents prettified byte values (base-2)
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[must_use]
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
}

impl ByteValuesBinary {
    // EiB is the max that can be represented with a u64
    const UNITS: [Self; 7] = [
        Self::B,
        Self::KiB,
        Self::MiB,
        Self::GiB,
        Self::TiB,
        Self::PiB,
        Self::EiB,
    ];
}

/// Convert a byte value to a "prettified" version
///
/// Converts using base-2 byte suffixes (KiB, MiB, GiB)
///
/// ## Example
/// ```
/// # use pretty_bytes_typed::pretty_bytes_binary;
/// // No rounding
/// let prettified = pretty_bytes_binary(1_048_576, None);
/// assert_eq!(prettified.to_string(), "1 MiB");
///
/// // Round to 2 decimal places
/// let prettified = pretty_bytes_binary(3_195_498, Some(2));
/// assert_eq!(prettified.to_string(), "3.05 MiB");
/// ```
// Most likely, values will be too small to experience precision loss, and they will often be rounded anyway
#[allow(clippy::cast_precision_loss)]
pub fn pretty_bytes_binary(num: u64, round_places: Option<u8>) -> PrettyBytesBinary {
    // Special handling for 0, because you can't use log on it
    if num == 0 {
        return PrettyBytesBinary {
            num: 0.,
            suffix: ByteValuesBinary::B,
        };
    }

    let exponent = std::cmp::min(num.ilog(1024) as usize, ByteValuesBinary::UNITS.len() - 1);

    let mut num = num as f64 / 1024_f64.powi(exponent as i32);

    if let Some(round_places) = round_places {
        num = round_float(num, round_places);
    }

    let unit = ByteValuesBinary::UNITS[exponent];

    PrettyBytesBinary { num, suffix: unit }
}

/// Convert a byte value to a "prettified" version, but accepts negative numbers
///
/// Converts using base-2 byte suffixes (KiB, MiB, GiB)
///
/// ## Example
/// ```
/// # use pretty_bytes_typed::pretty_bytes_signed_binary;
/// let prettified = pretty_bytes_signed_binary(-1_048_576, None);
/// assert_eq!(prettified.to_string(), "-1 MiB");
/// ```
pub fn pretty_bytes_signed_binary(num: i64, round_places: Option<u8>) -> PrettyBytesBinary {
    let is_negative = num.is_negative();
    let num = num.unsigned_abs();

    let mut pretty_bytes = pretty_bytes_binary(num, round_places);

    if is_negative {
        pretty_bytes.num = -pretty_bytes.num;
    }

    pretty_bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_pretty_bytes_binary() {
        // Test '0'
        assert_eq!(
            pretty_bytes_binary(0, None),
            PrettyBytesBinary {
                num: 0.,
                suffix: ByteValuesBinary::B,
            }
        );

        // Test all unit values
        assert_eq!(
            pretty_bytes_binary(1024_u64.pow(0), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::B,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_u64.pow(1), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::KiB,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_u64.pow(2), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::MiB,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_u64.pow(3), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::GiB,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_u64.pow(4), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::TiB,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_u64.pow(5), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::PiB,
            }
        );

        assert_eq!(
            pretty_bytes_binary(1024_u64.pow(6), None),
            PrettyBytesBinary {
                num: 1.,
                suffix: ByteValuesBinary::EiB,
            }
        );

        // Test rounding
        assert_eq!(
            pretty_bytes_binary(5014, Some(2)),
            PrettyBytesBinary {
                num: 4.9,
                suffix: ByteValuesBinary::KiB,
            }
        );
    }
}
