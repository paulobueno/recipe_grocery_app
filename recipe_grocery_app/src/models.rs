use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RecipeUrl {
    pub url: String,
}
