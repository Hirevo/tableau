//!
//! Example program that simply displays a table with multi-line cell contents.
//!

use owo_colors::OwoColorize;
use tableau::*;

fn main() {
    let table = Table::new()
        .with_style(Style::fancy())
        .with_row(
            Row::new()
                .with_cell(Cell::new("CLIENT ID".bold()).with_alignment(Alignment::Center))
                .with_cell(Cell::new("CLIENT NAME".bold()).with_alignment(Alignment::Center))
                .with_cell(Cell::new("GRANT TYPES".bold()).with_alignment(Alignment::Center))
                .with_cell(Cell::new("SCOPES".bold()).with_alignment(Alignment::Center)),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("58d4b31cebc380011bf6961d7d05a40a"))
                .with_cell(Cell::new("AwesomeService"))
                .with_cell(Cell::new(
                    "authorization_code\nclient_credentials\nrefresh_token",
                ))
                .with_cell(Cell::new("openid\nprofile")),
        )
        .with_row(
            Row::new()
                .with_cell(Cell::new("fb77c0f0ffccf92905c090cd6647d8eb"))
                .with_cell(Cell::new("FabulousProject"))
                .with_cell(Cell::new("authorization_code\nimplicit"))
                .with_cell(Cell::new("openid\nnickname\nusername")),
        );

    println!("{}", table.render());
}
