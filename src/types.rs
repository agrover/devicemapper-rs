// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use consts::SECTOR_SIZE;

use std::fmt;
use std::fmt::Display;
use std::ops::{Div, Mul, Rem};

use serde;
// macros for unsigned operations on Sectors and Bytes
macro_rules! unsigned_div {
    ($t: ty, $T: ident) => {
        impl Div<$t> for $T {
            type Output = $T;
            fn div(self, rhs: $t) -> $T {
                $T(self.0 / rhs as u64)
            }
        }
    }
}

macro_rules! unsigned_mul {
    ($t: ty, $T: ident) => {
        impl Mul<$t> for $T {
            type Output = $T;
            fn mul(self, rhs: $t) -> $T {
                $T(self.0 * rhs as u64)
            }
        }

        impl Mul<$T> for $t {
            type Output = $T;
            fn mul(self, rhs: $T) -> $T {
                $T(self as u64 * rhs.0)
            }
        }
    }
}

macro_rules! unsigned_rem {
    ($t: ty, $T: ident) => {
        impl Rem<$t> for $T {
            type Output = $T;
            fn rem(self, rhs: $t) -> $T {
                $T(self.0 % rhs as u64)
            }
        }
    }
}

// A type for Data Blocks as used by the thin pool.
custom_derive! {
    #[derive(NewtypeAdd, NewtypeAddAssign,
             NewtypeDeref,
             NewtypeFrom,
             NewtypeSub,
             Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
    pub struct DataBlocks(pub u64);
}

impl serde::Serialize for DataBlocks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_u64(**self)
    }
}

impl serde::Deserialize for DataBlocks {
    fn deserialize<D>(deserializer: D) -> Result<DataBlocks, D::Error>
        where D: serde::de::Deserializer
    {
        let val = try!(serde::Deserialize::deserialize(deserializer));
        Ok(DataBlocks(val))
    }
}

custom_derive! {
    #[derive(NewtypeAdd, NewtypeAddAssign,
             NewtypeDeref,
             NewtypeFrom,
             NewtypeSub,
             Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
    /// Structure to represent bytes
    pub struct Bytes(pub u64);
}

impl Bytes {
    /// Return the number of Sectors fully contained in these bytes.
    pub fn sectors(self) -> Sectors {
        Sectors(self.0 / SECTOR_SIZE as u64)
    }
}

unsigned_mul!(u64, Bytes);
unsigned_mul!(u32, Bytes);
unsigned_mul!(u16, Bytes);
unsigned_mul!(u8, Bytes);
unsigned_mul!(usize, Bytes);

impl Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} bytes", self.0)
    }
}

custom_derive! {
    #[derive(NewtypeAdd, NewtypeAddAssign,
             NewtypeDeref,
             NewtypeFrom,
             NewtypeSub,
             Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
    pub struct Sectors(pub u64);
}

impl Sectors {
    /// The number of bytes in these sectors.
    pub fn bytes(&self) -> Bytes {
        Bytes(self.0 * SECTOR_SIZE as u64)
    }
}

impl serde::Serialize for Sectors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_u64(**self)
    }
}

impl serde::Deserialize for Sectors {
    fn deserialize<D>(deserializer: D) -> Result<Sectors, D::Error>
        where D: serde::de::Deserializer
    {
        let val = try!(serde::Deserialize::deserialize(deserializer));
        Ok(Sectors(val))
    }
}

unsigned_div!(u64, Sectors);
unsigned_div!(u32, Sectors);
unsigned_div!(u16, Sectors);
unsigned_div!(u8, Sectors);
unsigned_div!(usize, Sectors);

unsigned_mul!(u64, Sectors);
unsigned_mul!(u32, Sectors);
unsigned_mul!(u16, Sectors);
unsigned_mul!(u8, Sectors);
unsigned_mul!(usize, Sectors);

unsigned_rem!(u64, Sectors);
unsigned_rem!(u32, Sectors);
unsigned_rem!(u16, Sectors);
unsigned_rem!(u8, Sectors);
unsigned_rem!(usize, Sectors);

impl Display for Sectors {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} sectors", self.0)
    }
}
