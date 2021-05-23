mod html_parser;
mod symbols;
use anyhow::Result;
use prettytable::Table;

#[tokio::main]
async fn main() -> Result<()> {
    let body = reqwest::get("https://www.istramet.hr/prognoza/pula/")
        .await?
        .text()
        .await?;
    let table = html_parser::parse_html(&body).await?;


    let mut table = Table::new();


    for v in table {
        println!("{:?}", v);
    }

    Ok(())
}
