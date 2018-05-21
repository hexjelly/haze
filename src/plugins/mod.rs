mod title;
pub use self::title::TitleLink;

use reqwest;

#[derive(Debug)]
pub enum PluginError {
    TitleError(String),
    ReqwestError(reqwest::Error),
    Unspecified,
}

impl From<reqwest::Error> for PluginError {
    fn from(e: reqwest::Error) -> Self {
        PluginError::ReqwestError(e)
    }
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
