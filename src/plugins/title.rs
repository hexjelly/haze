use super::super::helpers::get_url;
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

fn get_title(url: &str) -> Result<String, PluginError> {
    let body = get_url(url)?;
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
