mod html_parser;
mod symbols;

use scraper::{Html, Selector};


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://www.istramet.hr/prognoza/pula/").await?;

    let body = res.text().await?;
    //println!("Body:\n\n{}", body);
    // let document = Html::parse_document(&body);
    //
    // // table table-striped
    // let selector = Selector::parse("table").unwrap();
    // let element = document.select(&selector).next().unwrap();
    // //println!("elementn:\n\n{:?}", element);
    //
    // let tr_selector = Selector::parse("tr").unwrap();
    // let td_selector = Selector::parse("td").unwrap();
    // let th_selector = Selector::parse("th").unwrap();
    //
    // for element in element.select(&tr_selector) {
    //     let th = element.select(&th_selector);
    //     for e in th {
    //         println!(" th val =  {:?}", e.inner_html());
    //     }
    //
    //
    //
    //     for e in element.select(&td_selector) {
    //         println!(" td val =  {:?}", e.inner_html());
    //     }
    //
    //     println!("ROW END")
    // }


    let table = html_parser::parse_html(&body).await.unwrap();


    for v in table {
          println!("{:?}", v);
    }

    Ok(())
}
