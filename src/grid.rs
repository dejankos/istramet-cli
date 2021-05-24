use prettytable::Cell;

use crate::html_parser::{Row, TableData};

pub fn create_table(table_data: TableData) -> prettytable::Table {
    let rows = table_data.into_iter().map(create_table_row).collect();

    prettytable::Table::init(rows)
}

fn create_table_row(row: Row) -> prettytable::Row {
    let cells = row.iter().map(create_row_cell).collect();

    prettytable::Row::new(cells)
}

fn create_row_cell(val: &Option<String>) -> Cell {
    let cell_data = match val {
        Some(v) => v.trim(),
        _ => "",
    };

    Cell::new(cell_data)
}
