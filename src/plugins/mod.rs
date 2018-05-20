mod title;
pub use self::title::title;

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
