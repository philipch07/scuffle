use nutype_enum::nutype_enum;

use crate::ffi::*;

nutype_enum! {
    /// Rounding methods used in FFmpeg's `av_rescale_rnd` function.
    ///
    /// These rounding modes determine how values are rounded during scaling operations.
    ///
    /// See the official FFmpeg documentation:
    /// <https://ffmpeg.org/doxygen/trunk/group__lavu__math__rational.html>
    pub enum AVRounding(i32) {
        /// Round **toward zero** (truncate fractional part).
        /// - **Example**: `2.9 -> 2`, `-2.9 -> -2`
        /// - **Equivalent to**: `AV_ROUND_ZERO`
        Zero = AV_ROUND_ZERO as i32,

        /// Round **away from zero**.
        /// - **Example**: `2.1 -> 3`, `-2.1 -> -3`
        /// - **Equivalent to**: `AV_ROUND_INF`
        AwayFromZero = AV_ROUND_INF as i32,

        /// Round **toward negative infinity**.
        /// - **Example**: `2.9 -> 2`, `-2.1 -> -3`
        /// - **Equivalent to**: `AV_ROUND_DOWN`
        Down = AV_ROUND_DOWN as i32,

        /// Round **toward positive infinity**.
        /// - **Example**: `2.1 -> 3`, `-2.9 -> -2`
        /// - **Equivalent to**: `AV_ROUND_UP`
        Up = AV_ROUND_UP as i32,

        /// Round to the **nearest integer**, with halfway cases rounded **away from zero**.
        /// - **Example**: `2.5 -> 3`, `-2.5 -> -3`
        /// - **Equivalent to**: `AV_ROUND_NEAR_INF`
        NearestAwayFromZero = AV_ROUND_NEAR_INF as i32,

        /// Pass `INT64_MIN` / `INT64_MAX` **unchanged** during rescaling.
        ///
        /// **Bitmask flag** (must be combined with another rounding mode).
        ///
        /// - **Example**:
        ///   ```c
        ///   av_rescale_rnd(3, 1, 2, AV_ROUND_UP | AV_ROUND_PASS_MINMAX);
        ///   ```
        /// - **Equivalent to**: `AV_ROUND_PASS_MINMAX`
        PassMinMax = AV_ROUND_PASS_MINMAX as i32,
    }
}

impl PartialEq<i32> for AVRounding {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}
