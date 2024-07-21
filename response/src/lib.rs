/// Represents an option for the response of a language model.
#[derive(Debug, Clone)]
pub struct ResponseOption {
    /// The discriminator for the response type (e.g., `Answer` or `Fail`).
    pub type_name: String,
    /// The scenario for the response (e.g., "You know the answer" or "You don't know the answer").
    pub scenario: String,
    /// The description of what the response represents (e.g., "The capital city of the received country" or "Explanation on why the capital city is not known").
    pub description: String,
    /// The schema for the response.
    pub schema: Vec<ResponseSchemaField>,
}

/// Represents a field in the schema of a response.
#[derive(Debug, Clone)]
pub struct ResponseSchemaField {
    /// Name of the field (e.g., "capital" for the capital city).
    pub name: String,
    /// Description of the field (e.g., "Capital city of the received country").
    pub description: String,
    /// Type of the field (e.g., "string" for a string).
    pub typ: String,
    /// Example of the field (e.g., "London" for the capital city).
    pub example: String,
}

pub trait ResponseOptions<T>
where
    T: serde::de::DeserializeOwned,
{
    fn options(&self) -> Vec<ResponseOption>;

    fn parse(&self, response: &str) -> Result<T, serde_json::Error> {
        serde_json::from_str(response)
    }
}
