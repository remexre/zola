/// The front matter properties common to pages and sections, used to configure rendering
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RenderFrontMatter {
    /// Whether to turn quotes into code blocks of a certain language. This is useful for Literate
    /// Haskell, in particular.
    pub quotes_to_code: Option<String>,
}

impl Default for RenderFrontMatter {
    fn default() -> RenderFrontMatter {
        RenderFrontMatter { quotes_to_code: None }
    }
}
