use super::PluginError;
use failure::Error;
use haze::middleware::{Command, IrcMessage, Message, MessageResult, Middleware, Requirements};
use helpers::get_url;
use kuchiki;
use kuchiki::traits::*;
use regex::*;

pub struct TitleLink;

impl Middleware for TitleLink {
    fn name(&self) -> String {
        "Link Title".into()
    }

    fn process(&self, msg: &mut Message) -> MessageResult {
        match msg.original.command {
            Command::PRIVMSG(ref _name, ref msg) => {
                if let Some(url) = process_msg(msg) {
                    get_title(&url).map(|s| Some(s))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    fn requires(&self) -> Vec<Requirements> {
        vec![]
    }
}

fn process_msg(msg: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?i)((?:https?://(?:www\.)*?|www\.))(?:\S+\.\S+)").unwrap();
    }

    if let Some(caps) = RE.captures(msg) {
        Some(caps.get(0).unwrap().as_str().into())
    } else {
        None
    }
}

fn get_title(url: &str) -> Result<String, Error> {
    // TODO: remove multiple spaces, newlines etc
    let body = get_url(url)?;
    let document = kuchiki::parse_html().one(body.as_str());
    if let Some(title) = document
        .select("title")
        .map_err(|_| PluginError::Unspecified)?
        .nth(0)
    {
        let as_node = title.as_node();
        if let Some(text_node) = as_node.first_child() {
            let text = text_node.as_text().unwrap().borrow();
            Ok(text.to_string())
        } else {
            Err(PluginError::TitleError(format!("No title found for {}", url)).into())
        }
    } else {
        Err(PluginError::TitleError(format!("No title found for {}", url)).into())
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
        let ddg = get_title("https://duckduckgo.com/").unwrap();
        let google = get_title("https://google.com/").unwrap();
        let yt = get_title("https://youtube.com/").unwrap();

        assert!(ddg.contains("DuckDuckGo"));
        assert!(google.contains("Google"));
        assert!(yt.contains("YouTube"));
    }

    #[test]
    fn regex_msg_processing() {
        let msg = "https link: https://google.com/";

        let result = process_msg(msg);
        assert_eq!("https://google.com/", result.unwrap());

        let msg = "link without http(s): www.google.com and some text after it too wow";

        let result = process_msg(msg);
        assert_eq!("www.google.com", result.unwrap());
    }

    #[test]
    fn full_msg_processing() {
        let irc_msg = IrcMessage::new(
            Some("haze"),
            "PRIVMSG",
            vec!["#channel"],
            Some("https link: https://google.com/"),
        ).unwrap();

        let mut haze_msg = Message::from(&irc_msg);

        let result = Middleware::process(&TitleLink, &mut haze_msg).unwrap();
        assert_eq!(Some("Google".to_owned()), result);
    }
}
