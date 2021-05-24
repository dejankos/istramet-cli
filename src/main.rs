use anyhow::Result;
use structopt::StructOpt;

use crate::grid::create_table;
use crate::html_parser::parse_html;

mod grid;
mod html_parser;
mod symbols;

const BASE_URL: &str = "https://www.istramet.hr/prognoza/";

#[derive(StructOpt, Debug)]
pub struct CliArgs {
    #[structopt(help = "City name", default_value = "porec")]
    city: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::from_args();

    let body = fetch_html(&args.city).await?;
    let table_data = parse_html(&body).await?;
    let table = create_table(table_data);
    table.print_tty(true);

    Ok(())
}

async fn fetch_html(city: &str) -> reqwest::Result<String> {
    reqwest::get(format!("{}{}", BASE_URL, city))
        .await?
        .text()
        .await
}
