use haze::middleware::{
    Command, IrcMessage, MWError, Message, MessageResult, Middleware, Requirements,
};
use helpers::get_url;
use kuchiki;
use kuchiki::traits::*;
use regex::*;

pub struct LinkTitle;

impl Middleware for LinkTitle {
    fn name(&self) -> String {
        "Link Title".into()
    }

    fn process(&self, msg: &mut Message) -> MessageResult {
        match msg.chain[0].1.command {
            Command::PRIVMSG(ref _name, ref msg) => {
                if let Some(url) = process_msg(msg) {
                    get_title(&url).map(|s| Some(format!("[Title] {}", s)))
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

fn get_title(url: &str) -> Result<String, MWError> {
    // TODO: remove multiple spaces, newlines etc
    let body = get_url(url)?;
    let document = kuchiki::parse_html().one(body.as_str());
    if let Some(title) = document
        .select("title")
        .map_err(|_| MWError::ProcessError {
            name: Middleware::name(&LinkTitle),
            error: "Unknown error parsing HTML body with kuchiki".into(),
        })?
        .nth(0)
    {
        let as_node = title.as_node();
        if let Some(text_node) = as_node.first_child() {
            let text = text_node.as_text().unwrap().borrow();
            return Ok(clean_title(&text.to_string()));
        }
    }

    Err(MWError::ProcessError {
        name: Middleware::name(&LinkTitle),
        error: format!("No title found for {}", url),
    })
}

fn clean_title(title: &str) -> String {
    title.split_whitespace().collect::<Vec<_>>().join(" ")
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
    fn cleans_title_correctly() {
        let title_1 = "here's two spaces  and a trailing one ";
        let title_2 = "   leading spaces and a few trailing ones    ";
        let title_3 =
            " a mix of spaces and newlines  \r\n \t   \r more text\n\r \r \r \n   and a bit more ";

        let result_1 = clean_title(title_1);
        let result_2 = clean_title(title_2);
        let result_3 = clean_title(title_3);
        assert_eq!("here's two spaces and a trailing one", result_1);
        assert_eq!("leading spaces and a few trailing ones", result_2);
        assert_eq!(
            "a mix of spaces and newlines more text and a bit more",
            result_3
        );
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

        let result = Middleware::process(&LinkTitle, &mut haze_msg).unwrap();
        assert_eq!("[Title] Google", result.unwrap());
    }
}
