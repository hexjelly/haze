mod title;
pub use self::title::TitleLink;

#[derive(Debug)]
pub enum PluginError {
    TitleError(String),
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
