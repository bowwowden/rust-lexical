//! The maximum digits that can be held in a u64 for a given radix without overflow.
//!
//! This is useful for 128-bit division and operations, since it can
//! reduces the number of inefficient, non-native operations.
//!
//! # Generation
//!
//! See `etc/div128.py` for the script to generate the divisors and the
//! constants, and the division algorithm.

#![cfg(any(feature = "parse", feature = "write"))]

use crate::assert::debug_assert_radix;

/// Calculate the maximum number of digits
/// number of digits processed by 128-bit division.
#[inline(always)]
pub fn u64_step(radix: u32) -> usize {
    debug_assert_radix(radix);

    if cfg!(feature = "radix") {
        match radix {
            2 => u64_step_2(),
            3 => u64_step_3(),
            4 => u64_step_4(),
            5 => u64_step_5(),
            6 => u64_step_6(),
            7 => u64_step_7(),
            8 => u64_step_8(),
            9 => u64_step_9(),
            10 => u64_step_10(),
            11 => u64_step_11(),
            12 => u64_step_12(),
            13 => u64_step_13(),
            14 => u64_step_14(),
            15 => u64_step_15(),
            16 => u64_step_16(),
            17 => u64_step_17(),
            18 => u64_step_18(),
            19 => u64_step_19(),
            20 => u64_step_20(),
            21 => u64_step_21(),
            22 => u64_step_22(),
            23 => u64_step_23(),
            24 => u64_step_24(),
            25 => u64_step_25(),
            26 => u64_step_26(),
            27 => u64_step_27(),
            28 => u64_step_28(),
            29 => u64_step_29(),
            30 => u64_step_30(),
            31 => u64_step_31(),
            32 => u64_step_32(),
            33 => u64_step_33(),
            34 => u64_step_34(),
            35 => u64_step_35(),
            36 => u64_step_36(),
            _ => unreachable!(),
        }
    } else if cfg!(feature = "power-of-two") {
        match radix {
            2 => u64_step_2(),
            4 => u64_step_4(),
            8 => u64_step_8(),
            10 => u64_step_10(),
            16 => u64_step_16(),
            32 => u64_step_32(),
            _ => unreachable!(),
        }
    } else {
        u64_step_10()
    }
}

// AUTO-GENERATED
// These functions were auto-generated by `etc/div128.py`.
// Do not edit them unless there is a good reason to.
// Preferably, edit the source code to generate the constants.

#[inline]
const fn u64_step_2() -> usize {
    64
}

#[inline]
const fn u64_step_3() -> usize {
    40
}

#[inline]
const fn u64_step_4() -> usize {
    32
}

#[inline]
const fn u64_step_5() -> usize {
    27
}

#[inline]
const fn u64_step_6() -> usize {
    24
}

#[inline]
const fn u64_step_7() -> usize {
    22
}

#[inline]
const fn u64_step_8() -> usize {
    21
}

#[inline]
const fn u64_step_9() -> usize {
    20
}

#[inline]
const fn u64_step_10() -> usize {
    19
}

#[inline]
const fn u64_step_11() -> usize {
    18
}

#[inline]
const fn u64_step_12() -> usize {
    17
}

#[inline]
const fn u64_step_13() -> usize {
    17
}

#[inline]
const fn u64_step_14() -> usize {
    16
}

#[inline]
const fn u64_step_15() -> usize {
    16
}

#[inline]
const fn u64_step_16() -> usize {
    16
}

#[inline]
const fn u64_step_17() -> usize {
    15
}

#[inline]
const fn u64_step_18() -> usize {
    15
}

#[inline]
const fn u64_step_19() -> usize {
    15
}

#[inline]
const fn u64_step_20() -> usize {
    14
}

#[inline]
const fn u64_step_21() -> usize {
    14
}

#[inline]
const fn u64_step_22() -> usize {
    14
}

#[inline]
const fn u64_step_23() -> usize {
    14
}

#[inline]
const fn u64_step_24() -> usize {
    13
}

#[inline]
const fn u64_step_25() -> usize {
    13
}

#[inline]
const fn u64_step_26() -> usize {
    13
}

#[inline]
const fn u64_step_27() -> usize {
    13
}

#[inline]
const fn u64_step_28() -> usize {
    13
}

#[inline]
const fn u64_step_29() -> usize {
    13
}

#[inline]
const fn u64_step_30() -> usize {
    13
}

#[inline]
const fn u64_step_31() -> usize {
    12
}

#[inline]
const fn u64_step_32() -> usize {
    12
}

#[inline]
const fn u64_step_33() -> usize {
    12
}

#[inline]
const fn u64_step_34() -> usize {
    12
}

#[inline]
const fn u64_step_35() -> usize {
    12
}

#[inline]
const fn u64_step_36() -> usize {
    12
}
