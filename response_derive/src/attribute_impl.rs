use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub(crate) struct ResponseAttribute {
    pub(crate) scenario: String,
    pub(crate) description: String,
}

#[derive(Debug, FromMeta)]
pub(crate) struct SchemaAttribute {
    pub(crate) field: String,
    pub(crate) description: String,
    pub(crate) example: String,
}
