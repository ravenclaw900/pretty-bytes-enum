use crate::util::round_float;

/// Struct that represents prettified byte values (base-10)
#[derive(PartialEq, Debug, Clone)]
#[must_use]
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
}

impl ByteValues {
    // EB is the max that can be represented with a u64
    const UNITS: [Self; 7] = [
        Self::B,
        Self::KB,
        Self::MB,
        Self::GB,
        Self::TB,
        Self::PB,
        Self::EB,
    ];
}

/// Convert a byte value to a "prettified" version
///
/// Converts using base-10 byte suffixes (KB, MB, GB)
///
/// ## Example
/// ```
/// # use pretty_bytes_typed::pretty_bytes;
/// // No rounding
/// let prettified = pretty_bytes(2_000_000, None);
/// assert_eq!(prettified.to_string(), "2 MB");
///
/// // Round to 3 decimal places
/// let prettified = pretty_bytes(3_564_234, Some(3));
/// assert_eq!(prettified.to_string(), "3.564 MB");
/// ```
// Most likely, values will be too small to experience precision loss, and they will often be rounded anyway
#[allow(clippy::cast_precision_loss)]
pub fn pretty_bytes(num: u64, round_places: Option<u8>) -> PrettyBytes {
    // Special handling for 0, because you can't use log10 on it
    if num == 0 {
        return PrettyBytes {
            num: 0.,
            suffix: ByteValues::B,
        };
    }

    let exponent = std::cmp::min((num.ilog10() / 3) as usize, ByteValues::UNITS.len() - 1);

    let mut num = num as f64 / 1000_f64.powi(exponent as i32);

    if let Some(round_places) = round_places {
        num = round_float(num, round_places);
    }

    let unit = ByteValues::UNITS[exponent];

    PrettyBytes { num, suffix: unit }
}

/// Convert a byte value to a "prettified" version, but accepts negative numbers
///
/// Converts using base-10 byte suffixes (KB, MB, GB)
///
/// ## Example
/// ```
/// # use pretty_bytes_typed::pretty_bytes_signed;
/// let prettified = pretty_bytes_signed(-2_000_000, None);
/// assert_eq!(prettified.to_string(), "-2 MB");
/// ```
pub fn pretty_bytes_signed(num: i64, round_places: Option<u8>) -> PrettyBytes {
    let is_negative = num.is_negative();
    let num = num.unsigned_abs();

    let mut pretty_bytes = pretty_bytes(num, round_places);

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
    fn test_pretty_bytes() {
        // Test '0'
        assert_eq!(
            pretty_bytes(0, None),
            PrettyBytes {
                num: 0.,
                suffix: ByteValues::B,
            }
        );

        assert_eq!(
            pretty_bytes(5_430, None),
            PrettyBytes {
                num: 5.43,
                suffix: ByteValues::KB,
            }
        );

        // Test all unit values
        assert_eq!(
            pretty_bytes(1000_u64.pow(0), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::B,
            }
        );

        assert_eq!(
            pretty_bytes(1000_u64.pow(1), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::KB,
            }
        );

        assert_eq!(
            pretty_bytes(1000_u64.pow(2), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::MB,
            }
        );

        assert_eq!(
            pretty_bytes(1000_u64.pow(3), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::GB,
            }
        );

        assert_eq!(
            pretty_bytes(1000_u64.pow(4), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::TB,
            }
        );

        assert_eq!(
            pretty_bytes(1000_u64.pow(5), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::PB,
            }
        );

        assert_eq!(
            pretty_bytes(1000_u64.pow(6), None),
            PrettyBytes {
                num: 1.,
                suffix: ByteValues::EB,
            }
        );

        // Test extra large values (near u64::MAX)
        assert_eq!(
            pretty_bytes(18_000_000_000_000_000_000, None),
            PrettyBytes {
                num: 18.,
                suffix: ByteValues::EB,
            }
        );

        // Various other tests
        assert_eq!(
            pretty_bytes(50060, None),
            PrettyBytes {
                num: 50.06,
                suffix: ByteValues::KB,
            }
        );

        assert_eq!(
            pretty_bytes(736_532_432, None),
            PrettyBytes {
                num: 736.532_432,
                suffix: ByteValues::MB,
            }
        );

        // Test rounding
        assert_eq!(
            pretty_bytes(5003, Some(2)),
            PrettyBytes {
                num: 5.,
                suffix: ByteValues::KB,
            }
        );

        assert_eq!(
            pretty_bytes(8_452_020, Some(2)),
            PrettyBytes {
                num: 8.45,
                suffix: ByteValues::MB,
            }
        );

        assert_eq!(
            pretty_bytes(55_700, Some(0)),
            PrettyBytes {
                num: 56.,
                suffix: ByteValues::KB,
            }
        );
    }
}
