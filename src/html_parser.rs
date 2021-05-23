use std::error::Error;
use std::ops::Deref;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use scraper::{ElementRef, Html, Selector};
use scraper::element_ref::Select;

use crate::symbols::{Weather, Wind};

const EMPTY_STR: String = String::new();

pub type Row = Vec<Option<String>>;
pub type Table = Vec<Row>;

enum Selectors {
    Table,
    TableRow,
    TableHeader,
    TableData,
}

impl Selectors {
    fn value(&self) -> Selector {
        match self {
            Selectors::Table => Selector::parse("table").unwrap(),
            Selectors::TableRow => Selector::parse("tr").unwrap(),
            Selectors::TableHeader => Selector::parse("th").unwrap(),
            Selectors::TableData => Selector::parse("td").unwrap()
        }
    }
}

pub async fn parse_html(html: &str) -> Result<Table> {
    let doc = Html::parse_document(html);
    if let Some(data_table) = doc.select(&Selectors::Table.value()).next() {
        Ok(
            data_table.select(&Selectors::TableRow.value())
                .fold(Vec::new(), |mut acc, tr| {
                    let header = parse_header_row(tr.select(&Selectors::TableHeader.value()));
                    let data = parse_data_row(tr.select(&Selectors::TableData.value()));

                    if !header.is_empty() {
                        acc.push(header);
                    }
                    if !data.is_empty() {
                        acc.push(data);
                    }

                    acc
                })
        )
    } else {
        bail!("")
    }
}

fn parse_header_row(header_row: Select) -> Row {
    header_row.into_iter()
        .filter(|e| !e.inner_html().is_empty())
        .map(|e| Some(e.inner_html()))
        .collect()
}

fn parse_data_row(data_row: Select) -> Row {
    let mut iter = data_row.into_iter();
    vec![
        parse(iter.next(), |e| unwrap_val(e)),
        parse(iter.next(), |e| map_weather_pic(e)),
        parse(iter.next(), |e| unwrap_val(e)),
        parse(iter.next(), |e| map_rain(e)),
        parse(iter.next(), |e| unwrap_val(e)),
        parse(iter.next(), |e| unwrap_val(e)),
        parse(iter.next(), |e| map_wind(e)),
        parse(iter.next(), |e| unwrap_val(e)),
    ]
}

fn unwrap_val(e: ElementRef) -> Option<String> {
    Some(e.inner_html())
}

fn map_weather_pic(e: ElementRef) -> Option<String> {
    e.inner_html().split(".png")
        .collect::<Vec<&str>>()
        .into_iter()
        .nth(0)
        .map_or_else(|| None, |s| {
            let w = s.chars()
                .into_iter()
                .rev()
                .take_while(|c| *c != '/')
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .into_iter()
                .rev()
                .collect::<Vec<String>>()
                .join("");

            Weather::from_str(&w)
                .map(|w| Some(w.value().to_string()))
                .unwrap_or(None)
        },
        )
}

fn map_rain(e: ElementRef) -> Option<String> {
    let html = e.inner_html();
    if html.contains("nbsp") {
        None
    } else {
        Some(html)
    }
}

fn map_wind(e: ElementRef) -> Option<String> {
    let tokens = e.inner_html()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let wind = parse_wind(&tokens)
        .map(|w| Some(w.value()))
        .unwrap_or(None);
    let wind_speed = parse_wind_speed(tokens);

    if let Some(wind_dir) = wind {
        Some(format!("{} {} m/s", wind_dir, wind_speed))
    } else {
        None
    }
}

fn parse_wind(tokens: &Vec<String>) -> Result<Wind> {
    tokens.iter()
        .find(|t| t.contains("_png"))
        .map(|wd| wd.split("_png").collect::<Vec<&str>>()[0])
        .map(|wd| Wind::from_str(&wd))
        .unwrap_or(Err(anyhow!("wind token not found!")))
}

fn parse_wind_speed(tokens: Vec<String>) -> String {
    // todo lol
    tokens.into_iter()
        .nth_back(1)
        .unwrap_or(EMPTY_STR)
        .chars()
        .into_iter()
        .rev()
        .take_while(|c| c.is_digit(10))
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn parse<F, T>(e: Option<ElementRef>, f: F) -> Option<T>
    where F: Fn(ElementRef) -> Option<T>
{
    match e {
        Some(e_ref) => f(e_ref),
        _ => None
    }
}