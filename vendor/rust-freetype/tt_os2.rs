// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use freetype::{FT_UShort, FT_Short, FT_ULong, FT_Byte};

pub struct TT_OS2 {
    version: FT_UShort,
    xAvgCharWidth: FT_Short,
    usWeightClass: FT_UShort,
    usWidthClass: FT_UShort,
    fsType: FT_Short,
    ySubscriptXSize: FT_Short,
    ySubscriptYSize: FT_Short,
    ySubscriptXOffset: FT_Short,
    ySubscriptYOffset: FT_Short,
    ySuperscriptXSize: FT_Short,
    ySuperscriptYSize: FT_Short,
    ySuperscriptXOffset: FT_Short,
    ySuperscriptYOffset: FT_Short,
    yStrikeoutSize: FT_Short,
    yStrikeoutPosition: FT_Short,
    sFamilyClass: FT_Short,

    panose: [FT_Byte, ..10],

    ulUnicodeRange1: FT_ULong, /* Bits 0-31   */
    ulUnicodeRange2: FT_ULong, /* Bits 32-63  */
    ulUnicodeRange3: FT_ULong, /* Bits 64-95  */
    ulUnicodeRange4: FT_ULong, /* Bits 96-127 */

    /* only version 1 tables */

    ulCodePageRange1: FT_ULong, /* Bits 0-31  */
    ulCodePageRange2: FT_ULong, /* Bits 32-63 */

    /* only version 2 tables */

    sxHeight: FT_Short,
    sCapHeight: FT_Short,
    usDefaultChar: FT_UShort,
    usBreakChar: FT_UShort,
    usMaxContext: FT_UShort,
}
