use std::cmp::Ordering;

use crate::cell::{Alignment, Cell};
use crate::style::Style;

/// A row within a table.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    /// The cells within the row.
    pub cells: Vec<Cell>,
    /// Whether the row should have a top border.
    pub has_top_border: bool,
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

impl Row {
    /// Creates a new row.
    pub fn new() -> Self {
        Self {
            cells: Vec::default(),
            has_top_border: true,
        }
    }

    /// Adds a cell to the row.
    pub fn with_cell(mut self, cell: Cell) -> Self {
        self.cells.push(cell);
        self
    }

    /// Adds multiple cells to the row.
    pub fn with_cells(mut self, cells: impl IntoIterator<Item = Cell>) -> Self {
        self.cells.extend(cells);
        self
    }

    /// Disables the top border for the row.
    pub fn without_top_border(mut self) -> Self {
        self.has_top_border = false;
        self
    }

    /// Renders the contents of the row within the given string.
    pub fn render_content(&self, rendered: &mut String, column_widths: &[usize], style: &Style) {
        let mut row_height = 0;
        let mut spanned_columns = 0;
        let mut wrapped_contents = Vec::new();

        for cell in self.cells.iter() {
            let width = column_widths
                .iter()
                .copied()
                .skip(spanned_columns)
                .take(cell.column_span)
                .sum::<usize>()
                + (cell.column_span - 1)
                - if cell.has_padding { 2 } else { 0 };
            let wrapped_content = cell.wrapped_content(width);
            row_height = row_height.max(wrapped_content.len());
            wrapped_contents.push(wrapped_content);
            spanned_columns += cell.column_span;
        }

        for line_index in 0..row_height {
            if line_index > 0 {
                rendered.push('\n');
            }
            spanned_columns = 0;
            for (cell, wrapped_content) in self.cells.iter().zip(wrapped_contents.iter()) {
                let width: usize = column_widths
                    .iter()
                    .copied()
                    .skip(spanned_columns)
                    .take(cell.column_span)
                    .sum::<usize>()
                    + (cell.column_span - 1)
                    - if cell.has_padding { 2 } else { 0 };

                rendered.push(style.vertical);

                if cell.has_padding {
                    rendered.push(' ');
                }

                if let Some(line) = wrapped_content.get(line_index) {
                    let line_width = crate::visible_width(line);
                    match cell.alignment {
                        Alignment::Left => {
                            rendered.push_str(line);
                            rendered.extend(std::iter::repeat_n(' ', width - line_width));
                        }
                        Alignment::Center => {
                            let padding = (width - line_width) / 2;
                            rendered.extend(std::iter::repeat_n(' ', padding));
                            rendered.push_str(line);
                            rendered.extend(std::iter::repeat_n(' ', width - line_width - padding));
                        }
                        Alignment::Right => {
                            rendered.extend(std::iter::repeat_n(' ', width - line_width));
                            rendered.push_str(line);
                        }
                    }
                } else {
                    rendered.extend(std::iter::repeat_n(' ', width));
                }

                if cell.has_padding {
                    rendered.push(' ');
                }

                spanned_columns += cell.column_span;
            }

            rendered.push(style.vertical);
        }
    }

    /// Renders the top border of the row within the given string.
    pub fn render_top_border(
        &self,
        rendered: &mut String,
        column_widths: &[usize],
        style: &Style,
        last_row: Option<&Self>,
    ) {
        rendered.push(if last_row.is_none() {
            style.top_left_corner
        } else {
            style.outer_left_vertical
        });

        let intersections = list_intersections(
            last_row
                .iter()
                .flat_map(|row| row.cells.iter().map(|cell| cell.column_span)),
            self.cells.iter().map(|cell| cell.column_span),
        );

        let mut spanned_columns = 0;
        let mut last_intersection = None;
        for (column_span, intersection) in intersections {
            if let Some(intersection) = last_intersection {
                rendered.push(intersection);
            }
            last_intersection = Some(match intersection {
                Intersection::Both => style.intersection,
                Intersection::Top => style.outer_bottom_horizontal,
                Intersection::Bottom => style.outer_top_horizontal,
            });
            let width = column_widths
                .iter()
                .copied()
                .skip(spanned_columns)
                .take(column_span)
                .sum::<usize>()
                + (column_span - 1);
            rendered.extend(std::iter::repeat_n(style.horizontal, width));
            spanned_columns += column_span;
        }

        rendered.push(if last_row.is_none() {
            style.top_right_corner
        } else {
            style.outer_right_vertical
        });
    }
}

#[derive(Debug, PartialEq)]
enum Intersection {
    Both,
    Top,
    Bottom,
}

/// Lists all the intersections between two sets of column spans.
fn list_intersections(
    top_column_spans: impl Iterator<Item = usize>,
    bottom_column_spans: impl Iterator<Item = usize>,
) -> impl Iterator<Item = (usize, Intersection)> {
    let mut top_iter = top_column_spans.peekable();
    let mut bottom_iter = bottom_column_spans.peekable();

    let mut top_remaining = top_iter.next();
    let mut bottom_remaining = bottom_iter.next();

    std::iter::from_fn(move || match (top_remaining, bottom_remaining) {
        (None, None) => None,
        (Some(top), None) => {
            top_remaining = top_iter.next();
            Some((top, Intersection::Top))
        }
        (None, Some(bottom)) => {
            bottom_remaining = bottom_iter.next();
            Some((bottom, Intersection::Bottom))
        }
        (Some(top), Some(bottom)) => match top.cmp(&bottom) {
            Ordering::Equal => {
                top_remaining = top_iter.next();
                bottom_remaining = bottom_iter.next();
                Some((top, Intersection::Both))
            }
            Ordering::Less => {
                top_remaining = top_iter.next();
                bottom_remaining = bottom_remaining.map(|bottom| bottom - top);
                Some((top, Intersection::Top))
            }
            Ordering::Greater => {
                bottom_remaining = bottom_iter.next();
                top_remaining = top_remaining.map(|top| top - bottom);
                Some((bottom, Intersection::Bottom))
            }
        },
    })
}
