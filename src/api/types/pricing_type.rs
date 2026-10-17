pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PricingType {
    Unit,
    Fixed,
    Tiered,
    GraduatedTiered,
    TieredPercentage,
    GraduatedPercentage,
    VolumePercentage,
    PrepaidTiered,
    PrepaidFixedTiered,
    Expression,
    Matrix,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PricingType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unit => serializer.serialize_str("unit"),
            Self::Fixed => serializer.serialize_str("fixed"),
            Self::Tiered => serializer.serialize_str("tiered"),
            Self::GraduatedTiered => serializer.serialize_str("graduated_tiered"),
            Self::TieredPercentage => serializer.serialize_str("tiered_percentage"),
            Self::GraduatedPercentage => serializer.serialize_str("graduated_percentage"),
            Self::VolumePercentage => serializer.serialize_str("volume_percentage"),
            Self::PrepaidTiered => serializer.serialize_str("prepaid_tiered"),
            Self::PrepaidFixedTiered => serializer.serialize_str("prepaid_fixed_tiered"),
            Self::Expression => serializer.serialize_str("expression"),
            Self::Matrix => serializer.serialize_str("matrix"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PricingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unit" => Ok(Self::Unit),
            "fixed" => Ok(Self::Fixed),
            "tiered" => Ok(Self::Tiered),
            "graduated_tiered" => Ok(Self::GraduatedTiered),
            "tiered_percentage" => Ok(Self::TieredPercentage),
            "graduated_percentage" => Ok(Self::GraduatedPercentage),
            "volume_percentage" => Ok(Self::VolumePercentage),
            "prepaid_tiered" => Ok(Self::PrepaidTiered),
            "prepaid_fixed_tiered" => Ok(Self::PrepaidFixedTiered),
            "expression" => Ok(Self::Expression),
            "matrix" => Ok(Self::Matrix),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PricingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "unit"),
            Self::Fixed => write!(f, "fixed"),
            Self::Tiered => write!(f, "tiered"),
            Self::GraduatedTiered => write!(f, "graduated_tiered"),
            Self::TieredPercentage => write!(f, "tiered_percentage"),
            Self::GraduatedPercentage => write!(f, "graduated_percentage"),
            Self::VolumePercentage => write!(f, "volume_percentage"),
            Self::PrepaidTiered => write!(f, "prepaid_tiered"),
            Self::PrepaidFixedTiered => write!(f, "prepaid_fixed_tiered"),
            Self::Expression => write!(f, "expression"),
            Self::Matrix => write!(f, "matrix"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
