use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct GenericResponse<T> {
    /// Indicates the number of objects in the `Data` array
    #[serde(rename = "Count")]
    #[serde(default)]
    pub count: i64,

    /// An array containing a list of objects returned by the endpoint
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: Vec<T>,

    /// Indicates the number of objects in the `Data` array
    #[serde(rename = "Total")]
    #[serde(default)]
    pub total: i64,
}