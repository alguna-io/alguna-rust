pub use crate::prelude::*;

/// Pricing model used by this cell
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MatrixPriceCellInputType {
    Unit,
    Fixed,
    Tiered,
    GraduatedTiered,
    TieredPercentage,
    GraduatedPercentage,
    VolumePercentage,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MatrixPriceCellInputType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unit => serializer.serialize_str("unit"),
            Self::Fixed => serializer.serialize_str("fixed"),
            Self::Tiered => serializer.serialize_str("tiered"),
            Self::GraduatedTiered => serializer.serialize_str("graduated_tiered"),
            Self::TieredPercentage => serializer.serialize_str("tiered_percentage"),
            Self::GraduatedPercentage => serializer.serialize_str("graduated_percentage"),
            Self::VolumePercentage => serializer.serialize_str("volume_percentage"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MatrixPriceCellInputType {
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
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MatrixPriceCellInputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "unit"),
            Self::Fixed => write!(f, "fixed"),
            Self::Tiered => write!(f, "tiered"),
            Self::GraduatedTiered => write!(f, "graduated_tiered"),
            Self::TieredPercentage => write!(f, "tiered_percentage"),
            Self::GraduatedPercentage => write!(f, "graduated_percentage"),
            Self::VolumePercentage => write!(f, "volume_percentage"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
