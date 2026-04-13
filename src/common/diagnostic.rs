use crate::common::file::Position;
use colorize::AnsiColor;
use std::io;

#[derive(Debug, Clone, Copy)]
/// Represents severity of a diagnostic emitted by the compiler. Errors automatically abort compilation.
/// Warnings may abort compilation depending on end-user configuration. Info diagnostics also do not abort
/// compilation but may be used to convery information to the end-user that they would otherwise not be
/// aware of like version changes or best practices.
pub enum DiagLevel {
    Error,
    Warning,
    Info,
}

impl DiagLevel {
    /// Returns an ASCII-colorized string containing the name of the diagnostic level to be used
    /// when rendering a diagnostic.
    pub(self) fn colorize(&self, diag_kind: &'static str, code: u8) -> String {
        match self {
            Self::Error => format!("[E{}] {}", code, diag_kind).red().bold(),
            Self::Warning => format!("[W{}] {}", code, diag_kind).yellow().bold(),
            Self::Info => format!("[I{}] {}", code, diag_kind).blue().bold(),
        }
    }
}

#[derive(Debug)]
/// Returns all information pertaining to a diagnostic kind, including it's error code, name, and the level.
pub struct DiagKindInfo {
    /// Error code used for quick lookup and book-keeping.
    pub code: u8,

    /// The name of the diagnostic kind, what to the be displayed to the user;
    /// "what is this error?"
    pub name: &'static str,

    /// The severity of this diagnostic kind;
    /// "does this error abort compilation?"
    pub level: DiagLevel,
}

#[derive(Debug, PartialEq)]
/// Any kind of diagnostic that might be emitted by the compiler.
pub enum DiagKind {
    InternalError,
    SyntaxError,
    LossyConversion,

    BadPractice,
}

impl DiagKind {
    /// Returns all information about a diagnostic kind.
    pub fn get_info(&self) -> DiagKindInfo {
        use DiagLevel::*;
        let (code, name, level) = match self {
            Self::InternalError => (0u8, "internal error", Error),
            Self::SyntaxError => (1u8, "syntax error", Error),

            Self::LossyConversion => (8u8, "lossy conversion", Warning),

            Self::BadPractice => (30u8, "bad practice", Info),

            _ => todo!(),
        };
        DiagKindInfo { code, name, level }
    }
}

// ------------------------------------------------------------------------------------------------------------------ //
// MARK: diagnostic
// ------------------------------------------------------------------------------------------------------------------ //

#[derive(Debug)]
/// Represents a diagnostic that should be emitted to the user when compilation is complete (or aborted).
/// Contains information about the kind of diagnostic, position of the diagnostic, and a message.
pub struct Diag {
    /// Where the diagnostic ocurred (specifically what should be highlighted/underlined).
    pub pos: Position,

    /// The specific kind of diagnostic.
    pub kind: DiagKind,

    /// The path of the file the diagnostic was emitted for.
    path: String,

    /// A hopefully helpful message for the end-user.
    msg: String,
}

impl Diag {
    /// Creates a new diagnostic from given information.
    pub fn new(path: &str, kind: DiagKind, pos: Position, msg: String) -> Self {
        Self {
            path: path.to_string(),
            kind,
            pos,
            msg: msg.into(),
        }
    }

    /// Renders the diagnostic using the given iostream handle.
    pub fn render<W: io::Write>(&self, file: &str, w: &mut W) -> io::Result<()> {
        let info = self.kind.get_info();
        let err_name = info.level.colorize(info.name, info.code);

        write!(w, "{}: {}\n", err_name, self.msg)?;

        let line_number = format!("{}", self.pos.y);
        let line_number_width = self.pos.y.checked_ilog10().unwrap_or(0) as usize + 1;
        let blank_gutter_spaces = " ".repeat(line_number_width + 2);

        let path_content = format!(
            "{} --> {}:{}:{}",
            " ".repeat(line_number_width),
            self.path,
            self.pos.y,
            self.pos.x
        )
        .blue();
        write!(w, "{}\n", path_content)?;

        let err_start = self.pos.offset;
        let err_stop = file
            .as_bytes()
            .iter()
            .position(|b| *b == b'\n')
            .unwrap_or(self.pos.end())
            .min(self.pos.end());

        let line_start = file
            .as_bytes()
            .iter()
            .rposition(|b| *b == b'\n')
            .map(|i| i + 1)
            .unwrap_or(0);

        let line_stop = file
            .as_bytes()
            .iter()
            .position(|b| *b == b'\n')
            .unwrap_or(file.len());

        // Create line content from raw bytes
        let line_content =
            std::str::from_utf8(&file.as_bytes()[line_start..line_stop]).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "byte content of underlying file was invalid",
                )
            })?;

        let underline_line = "^".repeat(err_stop - err_start + 1);
        let underline_content = {
            let squiggles = " ".repeat(err_start - line_start + 1).to_string() + &underline_line;

            match info.level {
                DiagLevel::Error => squiggles.red(),
                DiagLevel::Info => squiggles.blue(),
                DiagLevel::Warning => squiggles.yellow(),
            }
        };

        // Line 1. Contains gutter, line number, and line content
        write!(
            w,
            " {} {} {}\n",
            line_number.blue(),
            "|".blue(),
            line_content
        )?;

        // Line 2. Contains gutter spaces and underline
        write!(w, "{} {}\n", blank_gutter_spaces, underline_content)?;

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------------------------ //
// MARK: tests
// ------------------------------------------------------------------------------------------------------------------ //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_error_diagnostic() {
        let source = "add(1, nan());";

        assert_eq!(source.as_bytes()[7], b'n');

        let diag = Diag::new(
            "render_error.sbl",
            DiagKind::SyntaxError,
            Position::new(7, 5, 6, 1),
            "expected `int`, got `NaN` instead.".to_string(),
        );

        let mut stderr = io::stderr().lock();
        diag.render(&source, &mut stderr).unwrap();
    }

    #[test]
    fn render_warning_diagnostic() {
        let source = "x := y as i16";

        assert_eq!(source.as_bytes()[5], b'y');

        let diag = Diag::new(
            "render_warning.sbl",
            DiagKind::LossyConversion,
            Position::new(5, 8, 6, 1),
            "conversion from `i32` to `i16` may result in lost information".to_string(),
        );

        let mut stderr = io::stderr().lock();
        diag.render(&source, &mut stderr).unwrap();
    }

    #[test]
    fn render_info_diagnostic() {
        let source = "z := this.read() ? 0 : -1;";

        assert_eq!(source.as_bytes()[5], b't');

        let diag = Diag::new(
            "render_info.sbl",
            DiagKind::BadPractice,
            Position::new(5, 20, 6, 1),
            concat!(
                "c-style ternary expressions are not preferred, try inline a `if a { b } else { c }` expression\n",
                "note: use `#supress(\"bad practice\")` to disable this diagnostic"
            ).to_string(),
        );

        let mut stderr = io::stderr().lock();
        diag.render(&source, &mut stderr).unwrap();
    }
}
