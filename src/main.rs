extern crate reqwest;
extern crate scraper;


use scraper::{Html, Selector};

fn main() {
    let url = "https://www.kubii.fr/cartes-raspberry-pi/2771-nouveau-raspberry-pi-4-modele-b-2gb-0765756931175.html?search_query=Pi4&results=111";
    println!("Welcome, the price of a raspebbty-pi 4 is :");
    scrape_price_data(url);
}


fn scrape_price_data(url: &str) {
    // cal url
    let mut req = reqwest::get(url).unwrap();
    assert!(req.status().is_success());

    let doc_body = Html::parse_document(&req.text().unwrap());
    let price = Selector::parse(r#"span[itemprop="price"]"#).unwrap();

    for price_rasp in doc_body.select(&price){
        let price = price_rasp.text().collect::<Vec<_>>();
        println!("{}", price[0]);
    }
}