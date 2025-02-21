use trpl::Html;

pub async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;

    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

pub async fn page_title_url(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());

    (url, title)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_page_title() {
        let content = page_title("https://google.com").await.unwrap();
        println!("{:?}", content)
    }

}