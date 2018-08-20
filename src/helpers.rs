use curl::easy::{Easy2, Handler, WriteError};
use haze::middleware::MWError;

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub(crate) fn get_url(url: &str) -> Result<String, MWError> {
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    easy.follow_location(true).unwrap();
    easy.max_filesize(5_000_000).unwrap();
    easy.url(url).unwrap();
    easy.perform().unwrap();

    let code = easy.response_code().unwrap();
    if code != 200 {
        return Err(MWError::ProcessError {
            name: "Generic".into(),
            error: format!("Error fetching URL with curl: {}", url),
        });
    }

    let body = easy.get_ref();
    Ok(String::from_utf8_lossy(&body.0).into_owned())
}
