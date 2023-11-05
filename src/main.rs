use::std::error::Error;
use serde::Deserialize;
use colour::{dark_green, yellow};

#[derive(Deserialize)]
struct Library
{
    articles: Vec<Article>
}

#[derive(Deserialize)]
struct Article
{
    title: String,
    url: String
}

fn get_articles(url: String) -> Result<Library, Box<dyn Error>>
{
    let response = ureq::get(url.as_str()).call()?.into_string()?;
    let articles: Library = serde_json::from_str(&response)?;

    Ok(articles)
}

fn render_articles(library: &Library)
{
    for article in &library.articles
    {
        dark_green!("> {}\n", article.title);
        yellow!("- {}\n\n", article.url);
    }
}

fn main() -> Result<(), Box<dyn Error>>
{
    let api_key = "API KEY";
    let mut url = format!("https://newsapi.org/v2/top-headlines?country=us&apiKey={}", api_key);

    let articles = get_articles(url)?;
    render_articles(&articles);

    Ok(())
}
