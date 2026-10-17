pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RevenueAllocationMethod {
    StraightLine,
    UsageBased,
    ContractLevelStraightLine,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RevenueAllocationMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StraightLine => serializer.serialize_str("straight_line"),
            Self::UsageBased => serializer.serialize_str("usage_based"),
            Self::ContractLevelStraightLine => {
                serializer.serialize_str("contract_level_straight_line")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RevenueAllocationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "straight_line" => Ok(Self::StraightLine),
            "usage_based" => Ok(Self::UsageBased),
            "contract_level_straight_line" => Ok(Self::ContractLevelStraightLine),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RevenueAllocationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StraightLine => write!(f, "straight_line"),
            Self::UsageBased => write!(f, "usage_based"),
            Self::ContractLevelStraightLine => write!(f, "contract_level_straight_line"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
