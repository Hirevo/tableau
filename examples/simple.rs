//!
//! Example program that simply displays a table with a maximum width, text-wrapping, and multiple cell alignments.
//!

use tableau::*;

fn main() {
    let table = Table::new()
        .with_style(Style::ascii())
        .with_max_column_width(40)
        .with_row(
            Row::new().with_cell(
                Cell::new("This is some centered text")
                    .with_column_span(2)
                    .with_alignment(Alignment::Center),
            ),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("This is left aligned text"))
                .with_cell(Cell::new("This is right aligned text").with_alignment(Alignment::Right)),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("This is left aligned text"))
                .with_cell(Cell::new("This is right aligned text").with_alignment(Alignment::Right)),
        )
        .with_row(
            Row::new().with_cell(
                Cell::new(
                    "This is some really really really really really really really really really long text that is going to wrap to the next line",
                )
                .with_column_span(2),
            ),
        );

    println!("{}", table.render());
}
