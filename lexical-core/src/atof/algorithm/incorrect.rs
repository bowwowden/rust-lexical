//! Incorrect, fast algorithms for string-to-float conversions.

use atoi;
use util::*;
use super::state::RawFloatState;
use lib::result::Result as StdResult;

// FRACTION

type Wrapped<F> = WrappedFloat<F>;

// Process the integer component of the raw float.
perftools_inline!{
fn process_integer<F: StablePower>(radix: u32, state: &RawFloatState)
    -> F
{
    match state.integer.len() {
        0 => F::ZERO,
        // This cannot error, since we cannot overflow and cannot have
        // invalid digits.
        _ => atoi::standalone_mantissa::<Wrapped<F>>(radix, state.integer)
                .unwrap()
                .0
                .into_inner()
    }
}}

// Process the fraction component of the raw float.
perftools_inline!{
fn process_fraction<F: StablePower>(radix: u32, state: &RawFloatState)
    -> F
{
    match state.fraction.len() {
        0 => F::ZERO,
        _ => {
            // We don't really care about numerical precision, so just break
            // the fraction into 12-digit pieces.
            // 12 is the maximum number of digits we can use without
            // potentially overflowing  a 36-radix float string.
            let mut fraction = F::ZERO;
            let mut digits: i32 = 0;
            for chunk in state.fraction.chunks(12) {
                digits = digits.saturating_add(chunk.len().as_i32());
                // This cannot error, since we have validated digits.
                let value: u64 = atoi::standalone_mantissa(radix, chunk).unwrap().0;
                if !value.is_zero() {
                    fraction += F::iterative_pow(as_cast(value), radix, -digits);
                }
            }
            fraction
        },
    }
}}

// Convert the float string to a native floating-point number.
perftools_inline!{
fn to_native<F: StablePower>(radix: u32, bytes: &[u8])
    -> StdResult<(F, *const u8), (ErrorCode, *const u8)>
{
    let mut state = RawFloatState::new();
    let ptr = state.parse(radix, bytes)?;

    let integer: F = process_integer(radix, &state);
    let fraction: F = process_fraction(radix, &state);
    let mut value = integer + fraction;
    if !state.exponent.is_zero() && !value.is_zero() {
        value = value.iterative_pow(radix, state.exponent);
    }
    Ok((value, ptr))
}}

// ATOF/ATOD
// ---------

// Parse 32-bit float from string.
perftools_inline!{
pub(crate) fn atof<'a>(radix: u32, bytes: &'a [u8], _: Sign)
    -> StdResult<(f32, *const u8), (ErrorCode, *const u8)>
{
    to_native::<f32>(radix, bytes)
}}

// Parse 64-bit float from string.
perftools_inline!{
pub(crate) fn atod<'a>(radix: u32, bytes: &'a [u8], _: Sign)
    -> StdResult<(f64, *const u8), (ErrorCode, *const u8)>
{
    to_native::<f64>(radix, bytes)
}}

// Parse 32-bit float from string.
perftools_inline!{
pub(crate) fn atof_lossy<'a>(radix: u32, bytes: &'a [u8], _: Sign)
    -> StdResult<(f32, *const u8), (ErrorCode, *const u8)>
{
    to_native::<f32>(radix, bytes)
}}

// Parse 64-bit float from string.
perftools_inline!{
pub(crate) fn atod_lossy<'a>(radix: u32, bytes: &'a [u8], _: Sign)
    -> StdResult<(f64, *const u8), (ErrorCode, *const u8)>
{
    to_native::<f64>(radix, bytes)
}}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    fn new_state<'a>(integer: &'a [u8], fraction: &'a [u8], exponent: i32)
        -> RawFloatState<'a>
    {
        RawFloatState { integer, fraction, exponent }
    }

    #[test]
    fn process_integer_test() {
        assert_eq!(1.0, process_integer::<f64>(10, &new_state(b"1", b"2345", 0)));
        assert_eq!(12.0, process_integer::<f64>(10, &new_state(b"12", b"345", 0)));
        assert_eq!(12345.0, process_integer::<f64>(10, &new_state(b"12345", b"6789", 0)));
    }

    #[test]
    fn process_fraction_test() {
        assert_eq!(0.2345, process_fraction::<f64>(10, &new_state(b"1", b"2345",0)));
        assert_eq!(0.345, process_fraction::<f64>(10, &new_state(b"12", b"345",0)));
        assert_eq!(0.6789, process_fraction::<f64>(10, &new_state(b"12345", b"6789",0)));
    }

    #[test]
    fn atof_test() {
        let atof10 = move |x| match atof(10, x, Sign::Positive) {
            Ok((v, p))  => Ok((v, distance(x.as_ptr(), p))),
            Err((v, p)) => Err((v, distance(x.as_ptr(), p))),
        };

        assert_eq!(Ok((1.2345, 6)), atof10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atof10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atof10(b"12345.6789"));
        assert_f32_eq!(1.2345e10, atof10(b"1.2345e10").unwrap().0);
    }

    #[test]
    fn atod_test() {
        let atod10 = move |x| match atod(10, x, Sign::Positive) {
            Ok((v, p))  => Ok((v, distance(x.as_ptr(), p))),
            Err((v, p)) => Err((v, distance(x.as_ptr(), p))),
        };

        assert_eq!(Ok((1.2345, 6)), atod10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atod10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atod10(b"12345.6789"));
        assert_f64_eq!(1.2345e10, atod10(b"1.2345e10").unwrap().0);
    }

    // Lossy
    // Just a synonym for the regular overloads, since we're not using the
    // correct feature. Use the same tests.

    #[test]
    fn atof_lossy_test() {
        let atof10 = move |x| match atof_lossy(10, x, Sign::Positive) {
            Ok((v, p))  => Ok((v, distance(x.as_ptr(), p))),
            Err((v, p)) => Err((v, distance(x.as_ptr(), p))),
        };

        assert_eq!(Ok((1.2345, 6)), atof10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atof10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atof10(b"12345.6789"));
        assert_f32_eq!(1.2345e10, atof10(b"1.2345e10").unwrap().0);
    }

    #[test]
    fn atod_lossy_test() {
        let atod10 = move |x| match atod_lossy(10, x, Sign::Positive) {
            Ok((v, p))  => Ok((v, distance(x.as_ptr(), p))),
            Err((v, p)) => Err((v, distance(x.as_ptr(), p))),
        };

        assert_eq!(Ok((1.2345, 6)), atod10(b"1.2345"));
        assert_eq!(Ok((12.345, 6)), atod10(b"12.345"));
        assert_eq!(Ok((12345.6789, 10)), atod10(b"12345.6789"));
        assert_f64_eq!(1.2345e10, atod10(b"1.2345e10").unwrap().0);
    }
}
