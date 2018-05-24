use super::PluginError;
use haze::middleware::{Message, MessageResult, Middleware, Requirements};
use kuchiki;
use kuchiki::traits::*;

pub struct TitleLink;

impl Middleware for TitleLink {
    fn name(&self) -> &str {
        "Link Title"
    }

    fn process(&self, msg: Option<Message>) -> MessageResult {
        if let Some(msg) = msg {
            let title = get_title("")?;
            Ok(Some(Message::new()))
        } else {
            Ok(None)
        }
    }

    fn requires(&self) -> Option<&[Requirements]> {
        None
    }
}

use curl::easy::{Easy2, Handler, WriteError};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn get_title(url: &str) -> Result<String, PluginError> {
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
    let body = String::from_utf8_lossy(&body.0).into_owned();

    let document = kuchiki::parse_html().one(body.as_str());
    if let Some(title) = document.select("title")?.nth(0) {
        let as_node = title.as_node();
        if let Some(text_node) = as_node.first_child() {
            let text = text_node.as_text().unwrap().borrow();
            Ok(text.to_string())
        } else {
            Err(PluginError::TitleError(format!(
                "No title found for {}",
                url
            )))
        }
    } else {
        Err(PluginError::TitleError(format!(
            "No title found for {}",
            url
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_title_http() {
        let ddg = get_title("http://duckduckgo.com/").unwrap();
        let google = get_title("http://google.com/").unwrap();

        assert!(ddg.contains("DuckDuckGo"));
        assert!(google.contains("Google"));
    }

    #[test]
    fn gets_title_https() {
        // let minimal_test_for_troubleshooting = reqwest::get("https://google.com").unwrap();

        let ddg = get_title("https://duckduckgo.com/").unwrap();
        let google = get_title("https://google.com/").unwrap();
        let yt = get_title("https://youtube.com/").unwrap();

        assert!(ddg.contains("DuckDuckGo"));
        assert!(google.contains("Google"));
        assert!(yt.contains("YouTube"));
    }

    // #[test]
    // fn errors_on_too_large_request() {
    //     unimplemented!()
    // }
}
