use reqwest; // For fetching web pages
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL of the webpage you want to parse
    let url = "https://example.com";
    
    // Fetch the webpage content
    let response = reqwest::get(url).await?.text().await?;
    
    // Parse the HTML
    let document = Html::parse_document(&response);
    
    // Create a selector for the specific tag you want to parse
    // Replace "div.classname" with your desired tag and class
    let selector = Selector::parse("div.classname").unwrap();
    
    // Find and extract content from all matching elements
    for element in document.select(&selector) {
        // Extract text content
        println!("Element text: {}", element.text().collect::<Vec<_>>().join(" "));
        
        // If you want to get the inner HTML
        println!("Inner HTML: {}", element.html());
        
        // If you want to get a specific attribute
        if let Some(id_attr) = element.value().attr("id") {
            println!("ID attribute: {}", id_attr);
        }
    }
    
    Ok(())
}

// Add these to your Cargo.toml:
// [dependencies]
// reqwest = { version = "0.11", features = ["json"] }
// scraper = "0.17"
// tokio = { version = "1", features = ["full"] }