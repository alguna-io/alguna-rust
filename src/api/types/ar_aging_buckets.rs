pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ArAgingBuckets {
    /// Balance not yet due
    pub current: String,
    /// Balance 1–30 days past due
    #[serde(rename = "days_1_to_30")]
    pub days1to30: String,
    /// Balance 31–60 days past due
    #[serde(rename = "days_31_to_60")]
    pub days31to60: String,
    /// Balance 61–90 days past due
    #[serde(rename = "days_61_to_90")]
    pub days61to90: String,
    /// Balance more than 90 days past due
    #[serde(rename = "days_90_plus")]
    pub days90plus: String,
}

impl ArAgingBuckets {
    pub fn builder() -> ArAgingBucketsBuilder {
        <ArAgingBucketsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArAgingBucketsBuilder {
    current: Option<String>,
    days1to30: Option<String>,
    days31to60: Option<String>,
    days61to90: Option<String>,
    days90plus: Option<String>,
}

impl ArAgingBucketsBuilder {
    pub fn current(mut self, value: impl Into<String>) -> Self {
        self.current = Some(value.into());
        self
    }

    pub fn days1to30(mut self, value: impl Into<String>) -> Self {
        self.days1to30 = Some(value.into());
        self
    }

    pub fn days31to60(mut self, value: impl Into<String>) -> Self {
        self.days31to60 = Some(value.into());
        self
    }

    pub fn days61to90(mut self, value: impl Into<String>) -> Self {
        self.days61to90 = Some(value.into());
        self
    }

    pub fn days90plus(mut self, value: impl Into<String>) -> Self {
        self.days90plus = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ArAgingBuckets`].
    /// This method will fail if any of the following fields are not set:
    /// - [`current`](ArAgingBucketsBuilder::current)
    /// - [`days1to30`](ArAgingBucketsBuilder::days1to30)
    /// - [`days31to60`](ArAgingBucketsBuilder::days31to60)
    /// - [`days61to90`](ArAgingBucketsBuilder::days61to90)
    /// - [`days90plus`](ArAgingBucketsBuilder::days90plus)
    pub fn build(self) -> Result<ArAgingBuckets, BuildError> {
        Ok(ArAgingBuckets {
            current: self
                .current
                .ok_or_else(|| BuildError::missing_field("current"))?,
            days1to30: self
                .days1to30
                .ok_or_else(|| BuildError::missing_field("days1to30"))?,
            days31to60: self
                .days31to60
                .ok_or_else(|| BuildError::missing_field("days31to60"))?,
            days61to90: self
                .days61to90
                .ok_or_else(|| BuildError::missing_field("days61to90"))?,
            days90plus: self
                .days90plus
                .ok_or_else(|| BuildError::missing_field("days90plus"))?,
        })
    }
}
