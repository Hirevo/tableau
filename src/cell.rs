use std::collections::HashSet;

/// The alignment of a cell's content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    /// Align the content to the left.
    Left,
    /// Align the content to the center.
    Center,
    /// Align the content to the right.
    Right,
}

/// A cell in a table row.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    /// The content of the cell.
    pub content: String,
    /// The number of columns the cell spans.
    pub column_span: usize,
    /// The alignment of the cell's content.
    pub alignment: Alignment,
    /// Whether the cell's content should be horizontally padded.
    pub has_padding: bool,
}

impl Cell {
    /// Creates a new cell with the given content.
    pub fn new(content: impl ToString) -> Self {
        Cell {
            content: content.to_string(),
            column_span: 1,
            alignment: Alignment::Left,
            has_padding: true,
        }
    }

    /// Sets the number of columns the cell spans.
    pub fn with_column_span(mut self, column_span: usize) -> Self {
        self.column_span = column_span;
        self
    }

    /// Sets the alignment of the cell's content.
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Disables horizontal padding for the cell.
    pub fn without_padding(mut self) -> Self {
        self.has_padding = false;
        self
    }

    /// Returns the width of the cell.
    pub(crate) fn width(&self) -> usize {
        crate::visible_width(&self.content) + if self.has_padding { 2 } else { 0 }
    }

    /// Returns the contents of the cell, wrapped to fit in `max_width`.
    pub(crate) fn wrapped_content(&self, max_width: usize) -> Vec<String> {
        let hidden: HashSet<usize> = crate::ANSI_REGEX
            .find_iter(&self.content)
            .flat_map(|m| m.start()..m.end())
            .collect();
        let mut res: Vec<String> = Vec::new();
        let mut buf = String::new();
        let mut byte_index = 0;
        for c in self.content.chars() {
            if !hidden.contains(&byte_index)
                && (crate::visible_width(&buf) >= max_width || c == '\n')
            {
                res.push(buf);
                buf = String::new();
                if c == '\n' {
                    byte_index += 1;
                    continue;
                }
            }
            byte_index += c.len_utf8();
            buf.push(c);
        }
        res.push(buf);

        res
    }
}
