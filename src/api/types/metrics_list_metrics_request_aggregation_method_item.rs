pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMetricsRequestAggregationMethodItem {
    Count,
    CountUnique,
    Average,
    Min,
    Max,
    Sum,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListMetricsRequestAggregationMethodItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Count => serializer.serialize_str("count"),
            Self::CountUnique => serializer.serialize_str("count_unique"),
            Self::Average => serializer.serialize_str("average"),
            Self::Min => serializer.serialize_str("min"),
            Self::Max => serializer.serialize_str("max"),
            Self::Sum => serializer.serialize_str("sum"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListMetricsRequestAggregationMethodItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "count" => Ok(Self::Count),
            "count_unique" => Ok(Self::CountUnique),
            "average" => Ok(Self::Average),
            "min" => Ok(Self::Min),
            "max" => Ok(Self::Max),
            "sum" => Ok(Self::Sum),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListMetricsRequestAggregationMethodItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Count => write!(f, "count"),
            Self::CountUnique => write!(f, "count_unique"),
            Self::Average => write!(f, "average"),
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
            Self::Sum => write!(f, "sum"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
