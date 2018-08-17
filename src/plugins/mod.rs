mod title;
pub use self::title::TitleLink;

#[derive(Debug, Fail)]
pub enum PluginError {
    #[fail(display = "TitleLink error")]
    TitleError(String),
    #[fail(display = "Unspecified plugin error")]
    Unspecified,
}

impl From<()> for PluginError {
    fn from(_e: ()) -> Self {
        PluginError::Unspecified
    }
}

impl From<PluginError> for String {
    fn from(e: PluginError) -> String {
        format!("{:?}", e)
    }
}
