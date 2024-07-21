use darling::FromMeta;

/// #[variant(...)]
#[derive(Debug, FromMeta)]
pub(crate) struct VariantAttribute {
    pub(crate) variant: String,
    pub(crate) scenario: String,
    pub(crate) description: String,
}

/// #[schema(...)]
#[derive(Debug, FromMeta)]
pub(crate) struct SchemaAttribute {
    pub(crate) description: String,
    pub(crate) example: String,
}
