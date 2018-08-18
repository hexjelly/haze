mod link_title;
pub use self::link_title::LinkTitle;

#[derive(Debug, Fail)]
pub enum PluginError {
    #[fail(display = "LinkTitle error")]
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
