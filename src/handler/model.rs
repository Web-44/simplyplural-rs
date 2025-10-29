use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct IdExists<T> {
    pub exists: bool,
    pub id: String,
    pub content: Option<T>,
}

#[cfg(any(feature = "users"))]
#[derive(Deserialize, Debug, Clone)]
pub struct ResultWithErrorMessage {
    pub success: bool,
    #[serde(rename = "msg")]
    pub message: Option<String>,
}

#[cfg(any(feature = "users", feature = "members"))]
#[derive(serde::Serialize, Deserialize, Debug, Clone)]
pub struct Frame {
    #[serde(rename = "bgShape")]
    background_shape: Option<String>,
    #[serde(rename = "bgClip")]
    background_clip: Option<String>,
    #[serde(rename = "bgStartColor")]
    background_start_color: Option<String>,
    #[serde(rename = "bgEndColor")]
    background_end_color: Option<String>,
}