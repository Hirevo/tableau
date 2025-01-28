#![deny(missing_docs)]

//! Tableau is a library for creating tables in Rust.

use std::collections::HashMap;
use std::sync::LazyLock;

mod cell;
mod row;
mod style;

use regex::Regex;
use unicode_width::UnicodeWidthStr;

pub use crate::cell::{Alignment, Cell};
pub use crate::row::Row;
pub use crate::style::Style;

/// The main struct for creating a table.
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    /// The rows in the table.
    pub rows: Vec<Row>,
    /// The style of the table.
    pub style: Style,
    /// The maximum width of all columns.
    ///
    /// It is overridden by the values in `max_column_widths`.
    pub max_column_width: Option<usize>,
    /// The maximum widths of specific columns.
    ///
    /// It overrides `max_column_width`.
    pub max_column_widths: HashMap<usize, usize>,
    /// Whether or not to vertically separate rows in the table.
    pub has_separate_rows: bool,
    /// Whether the table should have a top border.
    ///
    /// Setting `has_top_border` to false on the first row will have the same effect as setting this to false.
    pub has_top_border: bool,
    /// Whether the table should have a bottom border.
    pub has_bottom_border: bool,
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Table {
    /// Creates a new empty table, with default settings.
    pub fn new() -> Self {
        Self {
            rows: Vec::default(),
            style: Style::rounded(),
            max_column_width: None,
            max_column_widths: HashMap::default(),
            has_separate_rows: true,
            has_top_border: true,
            has_bottom_border: true,
        }
    }

    /// Adds a row to the table.
    pub fn with_row(mut self, row: Row) -> Self {
        self.rows.push(row);
        self
    }

    /// Adds multiple rows to the table.
    pub fn with_rows(mut self, rows: impl IntoIterator<Item = Row>) -> Self {
        self.rows.extend(rows);
        self
    }

    /// Sets the style of the table.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets the maximum width for all columns.
    pub fn with_max_column_width(mut self, max_column_width: usize) -> Self {
        self.max_column_width = Some(max_column_width);
        self
    }

    /// Sets the maximum width for a specific column.
    pub fn with_max_column_width_at_index(mut self, index: usize, max_column_width: usize) -> Self {
        self.max_column_widths.insert(index, max_column_width);
        self
    }

    /// Disables the vertical separation of rows.
    pub fn without_separate_rows(mut self) -> Self {
        self.has_separate_rows = false;
        self
    }

    /// Disables the top border of the table.
    pub fn without_top_border(mut self) -> Self {
        self.has_top_border = false;
        self
    }

    /// Disables the bottom border of the table.
    pub fn without_bottom_border(mut self) -> Self {
        self.has_bottom_border = false;
        self
    }

    /// Calculates the widths of the columns in the table.
    fn calculate_column_widths(&self) -> Vec<usize> {
        let max_number_of_columns = self
            .rows
            .iter()
            .map(|row| row.cells.iter().map(|cell| cell.column_span).sum::<usize>())
            .max()
            .unwrap_or(0);

        let mut column_widths = vec![0; max_number_of_columns];

        for row in self.rows.iter() {
            let mut spanned_columns = 0;
            for cell in row.cells.iter() {
                let cell_whole_width = cell.width();
                let subcell_width = (cell_whole_width - cell.column_span + 1) / cell.column_span;
                let leftover_width = (cell_whole_width - cell.column_span + 1) % cell.column_span;
                for (index, column_width) in column_widths
                    .iter_mut()
                    .skip(spanned_columns)
                    .take(cell.column_span)
                    .enumerate()
                {
                    *column_width = (*column_width).max(if index == 0 {
                        subcell_width + leftover_width
                    } else {
                        subcell_width
                    });
                }
                spanned_columns += cell.column_span;
            }
        }

        for (index, column_width) in column_widths.iter_mut().enumerate() {
            if let Some(max_width) = self
                .max_column_widths
                .get(&index)
                .copied()
                .or(self.max_column_width)
            {
                *column_width = (*column_width).min(max_width);
            }
        }

        column_widths
    }

    /// Renders the bottom border of the table.
    fn render_bottom_border(&self, rendered: &mut String, column_widths: &[usize], last_row: &Row) {
        let mut spanned_columns = 0;
        rendered.push(self.style.bottom_left_corner);
        for (index, cell) in last_row.cells.iter().enumerate() {
            if index != 0 {
                rendered.push(self.style.outer_bottom_horizontal);
            }
            let width = column_widths
                .iter()
                .copied()
                .skip(spanned_columns)
                .take(cell.column_span)
                .sum::<usize>()
                + (cell.column_span - 1);
            rendered.extend(std::iter::repeat_n(self.style.horizontal, width));
            spanned_columns += cell.column_span;
        }
        rendered.push(self.style.bottom_right_corner);
    }

    /// Renders the table to a string.
    pub fn render(&self) -> String {
        let mut rendered = String::new();
        let column_widths = self.calculate_column_widths();

        let mut last_row = None;
        for (index, row) in self.rows.iter().enumerate() {
            if last_row.is_some() {
                rendered.push('\n');
            }
            if row.has_top_border
                && ((index == 0 && self.has_top_border) || (index != 0 && self.has_separate_rows))
            {
                row.render_top_border(&mut rendered, &column_widths, &self.style, last_row);
                rendered.push('\n');
            }
            row.render_content(&mut rendered, &column_widths, &self.style);
            last_row = Some(row);
        }

        if self.has_bottom_border {
            rendered.push('\n');
            self.render_bottom_border(&mut rendered, &column_widths, self.rows.last().unwrap());
        }

        rendered
    }
}

pub(crate) static ANSI_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[\x1b\x9b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-PRZcf-nqry=><]").unwrap()
});

pub(crate) fn visible_width(s: &str) -> usize {
    ANSI_REGEX
        .replace_all(s, "")
        .lines()
        .map(|line| line.width())
        .max()
        .unwrap_or(0)
}
