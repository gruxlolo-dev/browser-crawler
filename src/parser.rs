use scraper::{Html, Selector};
use url::Url;

pub fn extract_links(base: &str, html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    let sel = Selector::parse("a").unwrap();

    doc.select(&sel)
        .filter_map(|e| e.value().attr("href"))
        .filter_map(|href| {
            Url::parse(base).ok()?.join(href).ok().map(|u| u.to_string())
        })
        .collect()
}
