use reqwest::Client;
use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use scraper::{Html, Selector};

async fn get_images() {
  let client = Client::new();
  let resp = client.get("https://www.theowlclub.net/category/episodios/primera-temporada/?order=asc").send().await.unwrap();
  
  // web_sys dom parser
  let fragment = Html::parse_fragment(&resp.text().await.unwrap());
  let mut images_links = vec![];

  let selector = Selector::parse("a.preview-image>img").unwrap();

  for node in fragment.select(&selector) {
    images_links.push(node.value().attr("src").unwrap())
  }

  println!("{:?}", images_links);

  for (i, link) in images_links.iter().enumerate() {
    // get the image
    let resp = client.get(*link).send().await.unwrap();

    // get the extension of the image
    let extension = link.split(".").last().unwrap();

    // we need to get the bytes of the image because a image is a file and a file is a byte array even the text files but for text files we can use the text() method and that way we don't need to convert the bytes to string manually
    let bytes = resp.bytes().await.unwrap();
    // save files in a specific folder

    // create a file if it doesn't exist
    if !std::path::Path::new("images").exists() {
      std::fs::create_dir("images").unwrap();
    }

    // create a file with the name of the index and the extension of the image
    let mut file = File::create(format!("images/{}.{}", i, extension)).await.unwrap();

    // write the bytes to the file
    file.write_all(&bytes).await.unwrap();

  } 
}


#[tokio::main]
async fn main() {
  
  // get images from a website
  get_images().await;

  // my function to add two numbers
  let add = | x: i32, y: i32 | x + y;
  println!("The result is {}", add(1, 2));
}