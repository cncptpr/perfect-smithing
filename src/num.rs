use std::{cmp::Ordering, fmt::Display, ops::Add};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Copy, Clone, Debug)]
pub enum Num {
    End(i64),
    Inf,
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::End(l0), Self::End(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::End(s), Self::End(o)) => Some(s.cmp(o)),
            (Self::Inf, Self::End(_)) => Some(Ordering::Greater),
            (Self::End(_), Self::Inf) => Some(Ordering::Less),
            (Self::Inf, Self::Inf) => None,
        }
    }
}

impl Add for Num {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::End(l), Self::End(r)) => Self::End(l + r),
            _ => Self::Inf,
        }
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::End(num) => write!(f, "{num}"),
            Num::Inf => write!(f, "∞"),
        }
    }
}

impl From<i64> for Num {
    fn from(value: i64) -> Self {
        Self::End(value)
    }
}

impl From<i32> for Num {
    fn from(value: i32) -> Self {
        Self::End(value as i64)
    }
}

impl Serialize for Num {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Num::End(n) => serializer.serialize_i64(*n),
            Num::Inf => serializer.serialize_i64(i64::MAX),
        }
    }
}

impl<'de> Deserialize<'de> for Num {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let n = i64::deserialize(deserializer)?;
        Ok(if n == i64::MAX { Num::Inf } else { Num::End(n) })
    }
}
