/// A struct that contains the characters used to draw a table.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Style {
    /// The character used for a top-left corner.
    pub top_left_corner: char,
    /// The character used for a top-right corner.
    pub top_right_corner: char,
    /// The character used for a bottom-left corner.
    pub bottom_left_corner: char,
    /// The character used for a bottom-right corner.
    pub bottom_right_corner: char,
    /// The character used for a vertical line with a right intersection.
    pub outer_left_vertical: char,
    /// The character used for a vertical line with a left intersection.
    pub outer_right_vertical: char,
    /// The character used for a horizontal line with a top intesection.
    pub outer_bottom_horizontal: char,
    /// The character used for a horizontal line with a bottom intersection.
    pub outer_top_horizontal: char,
    /// The character used for an four-way intersection of lines.
    pub intersection: char,
    /// The character used for a vertical line.
    pub vertical: char,
    /// The character used for a horizontal line.
    pub horizontal: char,
}

impl Style {
    /// A simple plain ASCII table style.
    ///
    /// <pre>
    /// +---------------------------------------------------------------------------------+
    /// |                            This is some centered text                           |
    /// +----------------------------------------+----------------------------------------+
    /// | This is left aligned text              |             This is right aligned text |
    /// +----------------------------------------+----------------------------------------+
    /// | This is left aligned text              |             This is right aligned text |
    /// +----------------------------------------+----------------------------------------+
    /// | This is some really really really really really really really really really tha |
    /// | t is going to wrap to the next line                                             |
    /// +---------------------------------------------------------------------------------+
    /// </pre>
    pub fn ascii() -> Style {
        Style {
            top_left_corner: '+',
            top_right_corner: '+',
            bottom_left_corner: '+',
            bottom_right_corner: '+',
            outer_left_vertical: '+',
            outer_right_vertical: '+',
            outer_bottom_horizontal: '+',
            outer_top_horizontal: '+',
            intersection: '+',
            vertical: '|',
            horizontal: '-',
        }
    }

    /// A heavy table style with double lines.
    ///
    /// <pre>
    /// ╔═════════════════════════════════════════════════════════════════════════════════╗
    /// ║                            This is some centered text                           ║
    /// ╠════════════════════════════════════════╦════════════════════════════════════════╣
    /// ║ This is left aligned text              ║             This is right aligned text ║
    /// ╠════════════════════════════════════════╬════════════════════════════════════════╣
    /// ║ This is left aligned text              ║             This is right aligned text ║
    /// ╠════════════════════════════════════════╩════════════════════════════════════════╣
    /// ║ This is some really really really really really really really really really tha ║
    /// ║ t is going to wrap to the next line                                             ║
    /// ╚═════════════════════════════════════════════════════════════════════════════════╝
    /// </pre>
    pub fn heavy() -> Style {
        Style {
            top_left_corner: '╔',
            top_right_corner: '╗',
            bottom_left_corner: '╚',
            bottom_right_corner: '╝',
            outer_left_vertical: '╠',
            outer_right_vertical: '╣',
            outer_bottom_horizontal: '╩',
            outer_top_horizontal: '╦',
            intersection: '╬',
            vertical: '║',
            horizontal: '═',
        }
    }

    /// A thin table style with single lines.
    ///
    /// <pre>
    /// ┌─────────────────────────────────────────────────────────────────────────────────┐
    /// │                            This is some centered text                           │
    /// ├────────────────────────────────────────┬────────────────────────────────────────┤
    /// │ This is left aligned text              │             This is right aligned text │
    /// ├────────────────────────────────────────┼────────────────────────────────────────┤
    /// │ This is left aligned text              │             This is right aligned text │
    /// ├────────────────────────────────────────┴────────────────────────────────────────┤
    /// │ This is some really really really really really really really really really tha │
    /// │ t is going to wrap to the next line                                             │
    /// └─────────────────────────────────────────────────────────────────────────────────┘
    /// </pre>
    pub fn thin() -> Style {
        Style {
            top_left_corner: '┌',
            top_right_corner: '┐',
            bottom_left_corner: '└',
            bottom_right_corner: '┘',
            outer_left_vertical: '├',
            outer_right_vertical: '┤',
            outer_bottom_horizontal: '┴',
            outer_top_horizontal: '┬',
            intersection: '┼',
            vertical: '│',
            horizontal: '─',
        }
    }

    /// A thin table style with single lines and rounded corners.
    ///
    /// <pre>
    /// ╭─────────────────────────────────────────────────────────────────────────────────╮
    /// │                            This is some centered text                           │
    /// ├────────────────────────────────────────┬────────────────────────────────────────┤
    /// │ This is left aligned text              │             This is right aligned text │
    /// ├────────────────────────────────────────┼────────────────────────────────────────┤
    /// │ This is left aligned text              │             This is right aligned text │
    /// ├────────────────────────────────────────┴────────────────────────────────────────┤
    /// │ This is some really really really really really really really really really tha │
    /// │ t is going to wrap to the next line                                             │
    /// ╰─────────────────────────────────────────────────────────────────────────────────╯
    /// </pre>
    pub fn rounded() -> Style {
        Style {
            top_left_corner: '╭',
            top_right_corner: '╮',
            bottom_left_corner: '╰',
            bottom_right_corner: '╯',
            outer_left_vertical: '├',
            outer_right_vertical: '┤',
            outer_bottom_horizontal: '┴',
            outer_top_horizontal: '┬',
            intersection: '┼',
            vertical: '│',
            horizontal: '─',
        }
    }

    /// A fancy table style with thin lines and prononced corners.
    ///
    /// <pre>
    /// ╔─────────────────────────────────────────────────────────────────────────────────╗
    /// │                            This is some centered text                           │
    /// ╠────────────────────────────────────────╦────────────────────────────────────────╣
    /// │ This is left aligned text              │             This is right aligned text │
    /// ╠────────────────────────────────────────┼────────────────────────────────────────╣
    /// │ This is left aligned text              │             This is right aligned text │
    /// ╠────────────────────────────────────────╩────────────────────────────────────────╣
    /// │ This is some really really really really really really really really really tha │
    /// │ t is going to wrap to the next line                                             │
    /// ╚─────────────────────────────────────────────────────────────────────────────────╝
    /// </pre>
    pub fn fancy() -> Style {
        Style {
            top_left_corner: '╔',
            top_right_corner: '╗',
            bottom_left_corner: '╚',
            bottom_right_corner: '╝',
            outer_left_vertical: '╠',
            outer_right_vertical: '╣',
            outer_bottom_horizontal: '╩',
            outer_top_horizontal: '╦',
            intersection: '┼',
            vertical: '│',
            horizontal: '─',
        }
    }

    /// A table style that does not draw any lines (using spaces).
    ///
    ///<pre>
    ///                           This is some centered text
    ///
    /// This is left aligned text                           This is right aligned text
    ///
    /// This is left aligned text                           This is right aligned text
    ///
    /// This is some really really really really really really really really really tha
    /// t is going to wrap to the next line
    ///</pre>
    pub fn empty() -> Style {
        Style {
            top_left_corner: ' ',
            top_right_corner: ' ',
            bottom_left_corner: ' ',
            bottom_right_corner: ' ',
            outer_left_vertical: ' ',
            outer_right_vertical: ' ',
            outer_bottom_horizontal: ' ',
            outer_top_horizontal: ' ',
            intersection: ' ',
            vertical: ' ',
            horizontal: ' ',
        }
    }
}
