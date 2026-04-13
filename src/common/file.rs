// ------------------------------------------------------------------------------------------------------------------ //
// MARK: position
// ------------------------------------------------------------------------------------------------------------------ //

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents some position in a source code file using a span implementation and tracking line/column
/// numbers. Positions are file-agnostic and do not hold references to their original source. Use carefully.
pub struct Position {
    /// The byte offset of this position.
    pub offset: usize,

    /// The byte length of this position.
    pub len: usize,

    /// 1-based column number.
    pub x: usize,

    /// 1-based line number.
    pub y: usize,
}

impl Position {
    /// Returns a new position given the information.
    pub const fn new(offset: usize, len: usize, x: usize, y: usize) -> Self {
        Self { offset, len, x, y }
    }

    /// Returns a range of bytes encoded by this position.
    pub const fn as_range(&self) -> std::ops::Range<usize> {
        self.offset..self.offset + self.len
    }

    /// Returns the index of the ending byte of this position.
    pub fn end(&self) -> usize {
        self.offset + self.len - 1
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        let a = self.end();
        let b = rhs.end();

        assert!(self.offset <= rhs.offset);
        assert!(b > a);

        let offset = self.offset;
        let len = b - a + 1;

        Self {
            offset,
            len,
            x: self.x,
            y: self.y,
        }
    }
}

// ------------------------------------------------------------------------------------------------------------------ //
// MARK: substring
// ------------------------------------------------------------------------------------------------------------------ //

#[derive(Debug, PartialEq, Eq, Hash)]
/// Used to represent some substring of source code without copying data or lugging around lifetimes
/// and references. Uses a simple span-based implemention. Source code is needed for comparison and retrieval.
/// 0-length substrings are valid, they are considered "empty".
pub struct Substring {
    /// The byte offset of where the substring starts.
    offset: usize,

    /// The byte length of the substring.
    len: usize,
}

impl Substring {
    /// Creates a new substring given an span-like `offset` and `len`.
    pub fn from_span(src: &str, offset: usize, len: usize) -> Self {
        debug_assert!(offset + len - 1 < src.len());
        debug_assert!(src.is_char_boundary(offset) && src.is_char_boundary(offset + len));
        Self { offset, len }
    }

    ///  Creates a new substring given a range of bytes to use.
    pub fn from_range(src: &str, range: std::ops::Range<usize>) -> Self {
        debug_assert!(range.end < src.len());
        debug_assert!(src.is_char_boundary(range.start) && src.is_char_boundary(range.end));
        Self {
            offset: range.start,
            len: range.len(),
        }
    }

    /// Converts the substring into a owned string and returns it.
    pub fn alloc_str(&self, src: &str) -> String {
        // We know from constricution that the span is within the source string, and that
        // it starts and ends on valid boundaries.
        src[self.range()].to_string()
    }

    /// Compares a substring with a static string and determines if they are the same data (not same position/length).
    pub fn compare_str(&self, src: &str, cmp: &str) -> bool {
        &src[self.range()] == cmp
    }

    /// Returns the range of bytes encoded by the substring.
    pub fn range(&self) -> std::ops::Range<usize> {
        self.offset..self.offset + self.len
    }

    /// Returns whether or not the length of the substring is 0. 0-length substrings are valid.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
