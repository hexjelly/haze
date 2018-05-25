use curl::easy::{Easy2, Handler, WriteError};
use plugins::PluginError;

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

// TODO: better error stuff, just copypasting code block from elsewhere lazily
pub(crate) fn get_url(url: &str) -> Result<String, PluginError> {
    // // old failing reqwest attempt
    // let client = reqwest::Client::new();
    // let resp = client.head(url).send()?;
    //
    // if !resp.status().is_success() {
    //     return Err(PluginError::TitleError(format!(
    //         "Error fetching header for: {}",
    //         url
    //     )));
    // }
    //
    // let len = resp.headers()
    //     .get::<reqwest::header::ContentLength>()
    //     .map(|ct_len| **ct_len)
    //     .unwrap_or(0);
    //
    // // limit 5mb response
    // if len > 5_000_000 {
    //     return Err(PluginError::TitleError(format!(
    //         "URL too large to fetch: {}",
    //         url
    //     )));
    // }
    //
    // let mut resp = client.get(url).send()?;
    // if !resp.status().is_success() {
    //     return Err(PluginError::TitleError(format!(
    //         "Error fetching page: {}",
    //         url
    //     )));
    // }

    // use curl instead for now
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    easy.follow_location(true).unwrap();
    easy.max_filesize(5_000_000).unwrap();
    easy.url(url).unwrap();
    easy.perform().unwrap();

    let code = easy.response_code().unwrap();
    if code != 200 {
        return Err(PluginError::TitleError(format!("Error fetching {}", url)));
    }

    let body = easy.get_ref();
    Ok(String::from_utf8_lossy(&body.0).into_owned())
}
