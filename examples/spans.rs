//!
//! Example program that simply displays a table with cells of multiple column spans.
//!

use owo_colors::OwoColorize;
use tableau::*;

fn main() {
    #[rustfmt::skip]
    let table = Table::new()
        .with_style(Style::heavy())
        .with_row(
            Row::new().with_cell(
                Cell::new("CELLS WITH MULTIPLE COLUMN SPANS".bold())
                    .with_alignment(Alignment::Center)
                    .with_column_span(4)
            ),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("FIRST".bold()).with_alignment(Alignment::Center))
                .with_cell(Cell::new("SECOND".bold()).with_alignment(Alignment::Center))
                .with_cell(Cell::new("THIRD".bold()).with_alignment(Alignment::Center))
                .with_cell(Cell::new("FOURTH".bold()).with_alignment(Alignment::Center)),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("One").with_alignment(Alignment::Center))
                .with_cell(Cell::new("One").with_alignment(Alignment::Center))
                .with_cell(Cell::new("One").with_alignment(Alignment::Center))
                .with_cell(Cell::new("One").with_alignment(Alignment::Center)),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("One").with_alignment(Alignment::Center))
                .with_cell(Cell::new("One").with_alignment(Alignment::Center))
                .with_cell(Cell::new("Two").with_alignment(Alignment::Center).with_column_span(2)),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("One").with_alignment(Alignment::Center))
                .with_cell(Cell::new("Two").with_alignment(Alignment::Center).with_column_span(2))
                .with_cell(Cell::new("One").with_alignment(Alignment::Center)),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("Two").with_alignment(Alignment::Center).with_column_span(2))
                .with_cell(Cell::new("One").with_alignment(Alignment::Center))
                .with_cell(Cell::new("One").with_alignment(Alignment::Center)),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("Two").with_alignment(Alignment::Center).with_column_span(2))
                .with_cell(Cell::new("Two").with_alignment(Alignment::Center).with_column_span(2)),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("One").with_alignment(Alignment::Center))
                .with_cell(Cell::new("Three").with_alignment(Alignment::Center).with_column_span(3)),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("Three").with_alignment(Alignment::Center).with_column_span(3))
                .with_cell(Cell::new("One").with_alignment(Alignment::Center)),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("Four").with_alignment(Alignment::Center).with_column_span(4)),
        );

    println!("{}", table.render());
}
