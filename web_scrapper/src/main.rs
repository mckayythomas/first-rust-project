use reqwest::Client; 
use scraper::{Html, Selector};
use tokio;

async fn fetch_data() {
    let client = Client::new();

    let res = client.get("http://books.toscrape.com/")
        .send()
        .await
        .unwrap();

    let body = res.text().await.expect("Failed to get response body");

    // Set document for parsing
    let document = Html::parse_document(&body);

    // Set book titles
    let book_title_selector = Selector::parse("h3 > a").unwrap();

    // Check book title
    for book_title in document.select(&book_title_selector) {
        let title = book_title.text().collect::<Vec<_>>();
        println!("Title: {}", title[0]);
    }

    // Get book price
    let book_price_selector = Selector::parse(".price_color").unwrap();

    // Check book price works
    for book_price in document.select(&book_price_selector) {
        let price = book_price.text().collect::<Vec<_>>();
        println!("Price: {}", price[0]);
    }

    // Group all data together.
    for (book_title, book_price) in document.select(&book_title_selector)
        .zip(document.select(&book_price_selector))
    {
        let title = book_title.text().collect::<Vec<_>>().join(" ");
        let price = book_price.text().collect::<Vec<_>>().join(" ");

        println!("Title: {} - Price: {}", title, price);
    }    



}


fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(fetch_data())
}