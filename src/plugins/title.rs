use super::PluginError;
use haze::middleware::Middleware;
use kuchiki;
use kuchiki::traits::*;
use reqwest;

pub fn title() -> Middleware {
    let middleware = Middleware::new("link title");

    middleware
}

fn get_title(url: &str) -> Result<String, PluginError> {
    let client = reqwest::Client::new();
    let resp = client.head(url).send()?;

    if !resp.status().is_success() {
        return Err(PluginError::TitleError(format!(
            "Error fetching header for: {}",
            url
        )));
    }

    let len = resp.headers()
        .get::<reqwest::header::ContentLength>()
        .map(|ct_len| **ct_len)
        .unwrap_or(0);

    // limit 5mb response
    if len > 5_000_000 {
        return Err(PluginError::TitleError(format!(
            "URL too large to fetch: {}",
            url
        )));
    }

    let mut resp = client.get(url).send()?;
    if !resp.status().is_success() {
        return Err(PluginError::TitleError(format!(
            "Error fetching page: {}",
            url
        )));
    }

    let document = kuchiki::parse_html().one(resp.text()?);
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
        let title = get_title("http://google.com").unwrap();
        assert_eq!(title, "Google");
    }

    #[test]
    fn gets_title_https() {
        let title = get_title("https://google.com").unwrap();
        assert_eq!(title, "Google");
    }

    // #[test]
    // fn errors_on_too_large_request() {
    //     unimplemented!()
    // }
}
