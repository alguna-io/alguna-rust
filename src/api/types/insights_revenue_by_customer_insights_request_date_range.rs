pub use crate::prelude::*;

/// Time period for the query. Determines the primary period and automatically calculates a comparison period.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RevenueByCustomerInsightsRequestDateRange {
    ThisMonth,
    LastMonth,
    TwoMonthsAgo,
    ThisQuarter,
    LastQuarter,
    TwoQuartersAgo,
    ThisYear,
    LastYear,
    LastTwelveMonths,
    AllTime,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RevenueByCustomerInsightsRequestDateRange {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ThisMonth => serializer.serialize_str("this_month"),
            Self::LastMonth => serializer.serialize_str("last_month"),
            Self::TwoMonthsAgo => serializer.serialize_str("two_months_ago"),
            Self::ThisQuarter => serializer.serialize_str("this_quarter"),
            Self::LastQuarter => serializer.serialize_str("last_quarter"),
            Self::TwoQuartersAgo => serializer.serialize_str("two_quarters_ago"),
            Self::ThisYear => serializer.serialize_str("this_year"),
            Self::LastYear => serializer.serialize_str("last_year"),
            Self::LastTwelveMonths => serializer.serialize_str("last_twelve_months"),
            Self::AllTime => serializer.serialize_str("all_time"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RevenueByCustomerInsightsRequestDateRange {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "this_month" => Ok(Self::ThisMonth),
            "last_month" => Ok(Self::LastMonth),
            "two_months_ago" => Ok(Self::TwoMonthsAgo),
            "this_quarter" => Ok(Self::ThisQuarter),
            "last_quarter" => Ok(Self::LastQuarter),
            "two_quarters_ago" => Ok(Self::TwoQuartersAgo),
            "this_year" => Ok(Self::ThisYear),
            "last_year" => Ok(Self::LastYear),
            "last_twelve_months" => Ok(Self::LastTwelveMonths),
            "all_time" => Ok(Self::AllTime),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RevenueByCustomerInsightsRequestDateRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThisMonth => write!(f, "this_month"),
            Self::LastMonth => write!(f, "last_month"),
            Self::TwoMonthsAgo => write!(f, "two_months_ago"),
            Self::ThisQuarter => write!(f, "this_quarter"),
            Self::LastQuarter => write!(f, "last_quarter"),
            Self::TwoQuartersAgo => write!(f, "two_quarters_ago"),
            Self::ThisYear => write!(f, "this_year"),
            Self::LastYear => write!(f, "last_year"),
            Self::LastTwelveMonths => write!(f, "last_twelve_months"),
            Self::AllTime => write!(f, "all_time"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
