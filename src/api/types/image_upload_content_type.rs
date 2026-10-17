pub use crate::prelude::*;

/// Media type of the bytes; it must match what the bytes actually are
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImageUploadContentType {
    ImagePng,
    ImageJpeg,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ImageUploadContentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ImagePng => serializer.serialize_str("image/png"),
            Self::ImageJpeg => serializer.serialize_str("image/jpeg"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ImageUploadContentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "image/png" => Ok(Self::ImagePng),
            "image/jpeg" => Ok(Self::ImageJpeg),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ImageUploadContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImagePng => write!(f, "image/png"),
            Self::ImageJpeg => write!(f, "image/jpeg"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
