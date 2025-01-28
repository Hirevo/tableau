//!
//! Example program that simply displays a table of cities, with longitudes and latitudes.
//!

use owo_colors::OwoColorize;
use tableau::*;

fn main() {
    let data = [
        ("Tokyo", "Japan", "139.6917", "35.6895"),
        ("Jakarta", "Indonesia", "106.8650", "-6.1751"),
        ("Delhi", "India", "77.1025", "28.7041"),
        ("Seoul", "South Korea", "126.9780", "37.5665"),
        ("Paris", "France", "2.3522", "48.8566"),
        ("London", "United Kingdom", "-0.1276", "51.5072"),
        ("Beijing", "China", "116.4074", "39.9042"),
        ("New York", "United States", "-74.0060", "40.7128"),
        ("Sao Paulo", "Brazil", "-46.6333", "-23.5505"),
        ("Mexico City", "Mexico", "-99.1332", "19.4326"),
        ("Mumbai", "India", "72.8777", "19.0760"),
        ("Los Angeles", "United States", "-118.2437", "34.0522"),
        ("Istanbul", "Turkey", "28.9784", "41.0082"),
    ];

    let table =
        Table::new()
            .with_row(
                Row::new().with_cell(
                    Cell::new("CITIES AROUND THE WORLD".bold())
                        .with_alignment(Alignment::Center)
                        .with_column_span(4),
                ),
            )
            .with_row(
                Row::new()
                    .with_cell(Cell::new("CITY".bold()).with_alignment(Alignment::Center))
                    .with_cell(Cell::new("COUNTRY".bold()).with_alignment(Alignment::Center))
                    .with_cell(Cell::new("LONGITUDE".bold()).with_alignment(Alignment::Center))
                    .with_cell(Cell::new("LATITUDE".bold()).with_alignment(Alignment::Center)),
            )
            .with_rows(data.iter().enumerate().map(
                |(index, (city, country, longitude, latitude))| {
                    let row = Row::new()
                        .with_cell(Cell::new(city))
                        .with_cell(Cell::new(country).with_alignment(Alignment::Center))
                        .with_cell(Cell::new(longitude).with_alignment(Alignment::Right))
                        .with_cell(Cell::new(latitude).with_alignment(Alignment::Right));

                    if index == 0 {
                        row
                    } else {
                        row.without_top_border()
                    }
                },
            ))
            .with_style(Style::rounded());

    println!("{}", table.render());
}
