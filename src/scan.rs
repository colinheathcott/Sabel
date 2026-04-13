use crate::common::{
    diagnostic::{Diag, DiagKind},
    file::Position,
    token::{TK, Token},
};

// ------------------------------------------------------------------------------------------------------------------ //
// MARK: scanner & api
// ------------------------------------------------------------------------------------------------------------------ //

pub struct Scanner<'a> {
    path: &'a str,
    input: &'a [u8],
    cursor: usize,
    cx: usize,
    cy: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(path: &'a str, input: &'a [u8]) -> Self {
        assert!(input.len() != 0, "empty input is invalid!");

        Self {
            path,
            input,
            cursor: 0,
            cx: 1,
            cy: 1,
        }
    }

    pub fn next(&mut self) -> Result<Token, Diag> {
        let token = self.token();

        // Advance the scanner internals to the start of the next token
        self.eat(1);

        // Then return the token to the caller
        return token;
    }
}

// ------------------------------------------------------------------------------------------------------------------ //
// MARK: scanner internals
// ------------------------------------------------------------------------------------------------------------------ //

impl<'a> Scanner<'a> {
    /// Returns the byte pointed at by `cursor`. Will return `0` when `cursor` points out of bounds.
    fn byte(&self) -> u8 {
        if self.cursor >= self.input.len() {
            0
        } else {
            self.input[self.cursor]
        }
    }

    /// Returns the byte pointed to by `cursor + 1`. Will return `0` when that points out of bounds.
    fn peek(&self) -> u8 {
        if self.cursor + 1 >= self.input.len() {
            0
        } else {
            self.input[self.cursor + 1]
        }
    }

    /// If the next byte is equal to `byte`, the scanner advances and returns `true`.
    /// Returns `false` otherwise.
    fn expect(&mut self, byte: u8) -> bool {
        if self.peek() == byte {
            self.eat(1);
            true
        } else {
            false
        }
    }

    /// Advances `cursor` and it's column counter `cx` by `k`.
    /// Use this cautiously -- there are no bounds check.
    fn eat(&mut self, k: usize) {
        self.cursor += k;
        self.cx += k;
    }

    /// Returns whether or not `byte` is a character that can start a symbol.
    fn is_symbol_start(byte: u8) -> bool {
        (byte as char).is_ascii_alphabetic() || byte == b'_'
    }

    /// Returns whether or not `byte` is a character that can continue a symbol.
    fn is_symbol_follow(byte: u8) -> bool {
        (byte as char).is_ascii_alphanumeric() || byte == b'_'
    }

    /// Returns whether or not `byte` is a character that can start a digit.
    fn is_digit_start(byte: u8) -> bool {
        (byte as char).is_ascii_digit()
    }

    /// Returns whether or not `byte` is a character that can continue a digit (including float literals)
    fn is_digit_follow(byte: u8) -> bool {
        (byte as char).is_ascii_digit() || byte == b'_' || byte == b'.'
    }

    /// Returns whether or not `byte` is a whitespace character.
    fn is_whitespace(byte: u8) -> bool {
        byte == b' ' || byte == b'\t'
    }

    // -------------------------------------------------------------------------------------------------------------- //
    // MARK: Scanner logic
    // -------------------------------------------------------------------------------------------------------------- //

    /// This method should end with cursor pointing at the last byte of the token it creates.
    fn token(&mut self) -> Result<Token, Diag> {
        let byte = self.byte();
        let cx = self.cx;
        let cy = self.cy;
        let c0 = self.cursor;

        // Skip whitespace
        while Self::is_whitespace(byte) {
            self.eat(1);
            return self.token();
        }

        // EOF
        if byte == 0 {
            return Ok(Token::new(TK::EOF, Position::new(c0, 0, cx, cy)));
        }

        // EOL (1)
        if byte == b'\r' {
            // Consume this but don't advance the column
            self.cursor += 1;

            // Make sure there's actually a newline
            // -- if there is, it will fall through and hit the next if block.
            if self.peek() != b'\n' {
                panic!("invalid source code");
            }
        }

        // EOL (2)
        if byte == b'\n' {
            self.cursor += 1;
            self.cy += 1;
            self.cx = 1;
            return self.token();
        }

        /* Handle symbol literals (and keywords)
         */
        if Self::is_symbol_start(byte) {
            while Self::is_symbol_follow(self.peek()) {
                self.eat(1);
            }

            // Look for keywords in this, otherwise assume it's a symbol
            // MARK: (match keywords)
            let kind = match &self.input[c0..self.cursor + 1] {
                // Keywords ...
                b"mut" => TK::Mut,
                b"fn" => TK::Fn,
                b"for" => TK::For,
                b"in" => TK::In,
                b"if" => TK::If,
                b"else" => TK::Else,
                b"while" => TK::While,
                b"defer" => TK::Defer,
                // Literals ...
                b"true" => TK::True,
                b"false" => TK::False,
                b"null" => TK::Null,
                _ => TK::Symbol,
            };

            return Ok(Token::new(
                kind,
                Position::new(c0, self.cursor - c0 + 1, cx, cy),
            ));
        }

        /* Handle digit literals
         */
        if Self::is_digit_start(byte) {
            let mut kind = TK::Integer;

            while Self::is_digit_follow(self.peek()) {
                self.eat(1);

                // If the current thing is a dot, we have to make sure it's actually
                // belonging to this token and backtrack if not...
                if self.byte() == b'.' {
                    if kind == TK::Integer && (self.peek() as char).is_ascii_digit() {
                        kind = TK::Float;
                        continue;
                    }

                    // If the above did not complete, then this dot is NOT for this token,
                    // so we need to backtrack.
                    self.cursor -= 1;
                    self.cx -= 1;
                    return Ok(Token::new(
                        kind,
                        Position::new(c0, self.cursor - c0 + 1, cx, cy),
                    ));
                }
            }

            return Ok(Token::new(
                kind,
                Position::new(c0, self.cursor - c0 + 1, cx, cy),
            ));
        }

        /* Handle string literals
         */
        if byte == b'"' {
            while self.peek() != b'"' {
                // Catch the EOF error
                if self.peek() == 0 {
                    // Emit syntax error
                    return Err(Diag::new(
                        self.path,
                        DiagKind::SyntaxError,
                        Position::new(self.cursor, 1, self.cx, self.cy),
                        "this string literal is missing a closing `\"`".to_string(),
                    ));
                }

                // In the future, string interpolation logic can fit here...

                // Otherwise consume the character
                self.eat(1);
            }

            // Consume the quote and return
            self.eat(1);
            return Ok(Token::new(
                TK::Str,
                Position::new(c0, self.cursor - c0 + 1, cx, cy),
            ));
        }

        /* Handle operators
         */
        match byte {
            /* Grouping operators
             */
            b'(' => Ok(Token::new(TK::LPar, Position::new(c0, 1, cx, cy))),
            b')' => Ok(Token::new(TK::RPar, Position::new(c0, 1, cx, cy))),
            b'[' => Ok(Token::new(TK::LBrac, Position::new(c0, 1, cx, cy))),
            b']' => Ok(Token::new(TK::RBrac, Position::new(c0, 1, cx, cy))),
            b'{' => Ok(Token::new(TK::LCurl, Position::new(c0, 1, cx, cy))),
            b'}' => Ok(Token::new(TK::RCurl, Position::new(c0, 1, cx, cy))),

            /* Arithmetic operators
             */
            b'+' => {
                if self.expect(b'=') {
                    Ok(Token::new(TK::PlusEq, Position::new(c0, 2, cx, cy)))
                } else if self.expect(b'+') {
                    Ok(Token::new(TK::PlusPlus, Position::new(c0, 2, cx, cy)))
                } else {
                    Ok(Token::new(TK::Plus, Position::new(c0, 1, cx, cy)))
                }
            }
            b'-' => {
                if self.expect(b'=') {
                    Ok(Token::new(TK::MinEq, Position::new(c0, 2, cx, cy)))
                } else if self.expect(b'-') {
                    Ok(Token::new(TK::MinMin, Position::new(c0, 2, cx, cy)))
                } else {
                    Ok(Token::new(TK::Min, Position::new(c0, 1, cx, cy)))
                }
            }
            b'*' => {
                if self.expect(b'*') {
                    if self.expect(b'=') {
                        Ok(Token::new(TK::StarStarEq, Position::new(c0, 3, cx, cy)))
                    } else {
                        Ok(Token::new(TK::StarStar, Position::new(c0, 3, cx, cy)))
                    }
                } else if self.expect(b'=') {
                    Ok(Token::new(TK::StarEq, Position::new(c0, 2, cx, cy)))
                } else {
                    Ok(Token::new(TK::Star, Position::new(c0, 1, cx, cy)))
                }
            }
            b'/' => {
                if self.expect(b'/') {
                    if self.expect(b'=') {
                        Ok(Token::new(TK::SlashSlashEq, Position::new(c0, 3, cx, cy)))
                    } else {
                        Ok(Token::new(TK::SlashSlash, Position::new(c0, 3, cx, cy)))
                    }
                } else if self.expect(b'=') {
                    Ok(Token::new(TK::SlashEq, Position::new(c0, 2, cx, cy)))
                } else {
                    Ok(Token::new(TK::Slash, Position::new(c0, 1, cx, cy)))
                }
            }
            b'%' => {
                if self.expect(b'=') {
                    Ok(Token::new(TK::ModEq, Position::new(c0, 2, cx, cy)))
                } else {
                    Ok(Token::new(TK::Mod, Position::new(c0, 1, cx, cy)))
                }
            }

            /* Comparison operators
             */
            b'!' => {
                if self.expect(b'=') {
                    Ok(Token::new(TK::BangEq, Position::new(c0, 2, cx, cy)))
                } else {
                    Ok(Token::new(TK::Bang, Position::new(c0, 1, cx, cy)))
                }
            }
            b'<' => {
                if self.expect(b'=') {
                    Ok(Token::new(TK::LtEq, Position::new(c0, 2, cx, cy)))
                } else {
                    Ok(Token::new(TK::Lt, Position::new(c0, 1, cx, cy)))
                }
            }
            b'>' => {
                if self.expect(b'=') {
                    Ok(Token::new(TK::GtEq, Position::new(c0, 2, cx, cy)))
                } else {
                    Ok(Token::new(TK::Gt, Position::new(c0, 1, cx, cy)))
                }
            }
            b'=' => {
                if self.expect(b'=') {
                    Ok(Token::new(TK::EqEq, Position::new(c0, 2, cx, cy)))
                } else {
                    Ok(Token::new(TK::Eq, Position::new(c0, 1, cx, cy)))
                }
            }

            /* Logical and bitwise operators
             */
            b'|' => {
                if self.expect(b'|') {
                    Ok(Token::new(TK::BarBar, Position::new(c0, 2, cx, cy)))
                } else {
                    Ok(Token::new(TK::Bar, Position::new(c0, 1, cx, cy)))
                }
            }
            b'&' => {
                if self.expect(b'&') {
                    Ok(Token::new(TK::AmpsandAmpsand, Position::new(c0, 2, cx, cy)))
                } else {
                    Ok(Token::new(TK::Ampsand, Position::new(c0, 1, cx, cy)))
                }
            }

            /* Misc operators
             */
            b'.' => Ok(Token::new(TK::Dot, Position::new(c0, 1, cx, cy))),
            b':' => Ok(Token::new(TK::Colon, Position::new(c0, 1, cx, cy))),
            b';' => Ok(Token::new(TK::Semicolon, Position::new(c0, 1, cx, cy))),

            _ => Err(Diag::new(
                self.path,
                DiagKind::SyntaxError,
                Position::new(c0, 1, cx, cy),
                format!("character `{}` is not allowed", byte as char),
            )),
        }
    }
}

// ------------------------------------------------------------------------------------------------------------------ //
// MARK:tests
// ------------------------------------------------------------------------------------------------------------------ //

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns a string slice  containing the lexeme given by the token.
    fn token_lexeme<'a>(src: &'a str, token: &Token) -> &'a str {
        &src[token.pos.offset..token.pos.offset + token.pos.len]
    }

    #[test]
    fn scan_newlines() {
        let input = "fn\nmut";
        let mut scanner = Scanner::new("scan_newlines.sbl", input.as_bytes());

        {
            let t = scanner.next().unwrap();
            assert_eq!(t.kind, TK::Fn);
            assert_eq!(t.pos.offset, 0);
            assert_eq!(t.pos.len, 2);
            assert_eq!(t.pos.y, 1);
            assert_eq!(t.pos.x, 1);
            assert_eq!(token_lexeme(&input, &t), "fn");
        }
        {
            let t = scanner.next().unwrap();
            assert_eq!(t.kind, TK::Mut);
            assert_eq!(t.pos.offset, 3);
            assert_eq!(t.pos.len, 3);
            assert_eq!(t.pos.y, 2);
            assert_eq!(t.pos.x, 1);
            assert_eq!(token_lexeme(&input, &t), "mut");
        }
    }

    #[test]
    fn scan_strings() {
        let input = "\"Sable!\"".to_string();
        let mut scanner = Scanner::new("scan_strings.sbl", input.as_bytes());

        {
            let t = scanner.next().unwrap();
            assert_eq!(t.kind, TK::Str);
            assert_eq!(t.pos.offset, 0);
            assert_eq!(t.pos.len, 8);
            assert_eq!(token_lexeme(&input, &t), "\"Sable!\"");
        }
    }

    #[test]
    fn scan_digits() {
        let input = "1 0.1  1._add 0.1._add".to_string();
        let mut scanner = Scanner::new("scan_digits.sbl", input.as_bytes());

        {
            let t = scanner.next().unwrap();
            assert_eq!(t.kind, TK::Integer);
            assert_eq!(t.pos.offset, 0);
            assert_eq!(t.pos.len, 1);
            assert_eq!(token_lexeme(&input, &t), "1");
        }
        {
            let t = scanner.next().unwrap();
            assert_eq!(t.kind, TK::Float);
            assert_eq!(t.pos.offset, 2);
            assert_eq!(t.pos.len, 3);
            assert_eq!(token_lexeme(&input, &t), "0.1");
        }
        {
            {
                let t = scanner.next().unwrap();
                assert_eq!(t.kind, TK::Integer);
                assert_eq!(t.pos.offset, 7);
                assert_eq!(t.pos.len, 1);
                assert_eq!(token_lexeme(&input, &t), "1");
            }
            {
                let t = scanner.next().unwrap();
                assert_eq!(t.kind, TK::Dot);
                assert_eq!(t.pos.offset, 8);
                assert_eq!(t.pos.len, 1);
            }
            {
                let t = scanner.next().unwrap();
                assert_eq!(t.kind, TK::Symbol);
                assert_eq!(t.pos.offset, 9);
                assert_eq!(t.pos.len, 4);
                assert_eq!(token_lexeme(&input, &t), "_add");
            }
        }
        {
            {
                let t = scanner.next().unwrap();
                assert_eq!(t.kind, TK::Float);
                assert_eq!(t.pos.offset, 14);
                assert_eq!(t.pos.len, 3);
                assert_eq!(token_lexeme(&input, &t), "0.1");
            }
            {
                let t = scanner.next().unwrap();
                assert_eq!(t.kind, TK::Dot);
                assert_eq!(t.pos.offset, 17);
                assert_eq!(t.pos.len, 1);
            }
            {
                let t = scanner.next().unwrap();
                assert_eq!(t.kind, TK::Symbol);
                assert_eq!(t.pos.offset, 18);
                assert_eq!(t.pos.len, 4);
                assert_eq!(token_lexeme(&input, &t), "_add");
            }
        }
    }

    #[test]
    fn scan_symbols() {
        let input = "main".to_string();
        let mut scanner = Scanner::new("scan_symbols.sbl", input.as_bytes());

        {
            let t = scanner.next().unwrap();
            assert_eq!(t.kind, TK::Symbol);
            assert_eq!(t.pos.offset, 0);
            assert_eq!(t.pos.len, 4);
            assert_eq!(token_lexeme(&input, &t), "main");
        }
    }

    #[test]
    fn scan_keywords() {
        let input = "for fn mut".to_string();
        let mut scanner = Scanner::new("scan_keywords.sbl", input.as_bytes());

        {
            let t = scanner.next().unwrap();
            assert_eq!(t.kind, TK::For);
            assert_eq!(t.pos.offset, 0);
            assert_eq!(t.pos.len, 3);
            assert_eq!(token_lexeme(&input, &t), "for");
        }
        {
            let t = scanner.next().unwrap();
            assert_eq!(t.kind, TK::Fn);
            assert_eq!(t.pos.offset, 4);
            assert_eq!(t.pos.len, 2);
            assert_eq!(token_lexeme(&input, &t), "fn");
        }
        {
            let t = scanner.next().unwrap();
            assert_eq!(t.kind, TK::Mut);
            assert_eq!(t.pos.offset, 7);
            assert_eq!(t.pos.len, 3);
            assert_eq!(token_lexeme(&input, &t), "mut");
        }
    }

    #[test]
    fn scan_operators() {
        let input = "+=+".to_string();
        let mut scanner = Scanner::new("scan_operators.sbl", input.as_bytes());

        {
            let t = scanner.next().unwrap();
            assert_eq!(t.kind, TK::PlusEq);
            assert_eq!(t.pos.offset, 0);
            assert_eq!(t.pos.len, 2);
            assert_eq!(t.pos.x, 1);
            assert_eq!(t.pos.y, 1);
        }
        {
            let t = scanner.next().unwrap();
            assert_eq!(t.kind, TK::Plus);
            assert_eq!(t.pos.offset, 2);
            assert_eq!(t.pos.len, 1);
            assert_eq!(t.pos.x, 3);
            assert_eq!(t.pos.y, 1);
        }
        {
            let t = scanner.next().unwrap();
            assert_eq!(t.kind, TK::EOF);
            assert_eq!(t.pos.offset, 3);
            assert_eq!(t.pos.len, 0);
            assert_eq!(t.pos.x, 4);
            assert_eq!(t.pos.y, 1);
        }
    }

    #[test]
    fn test_illegal_character() {
        let source = "$";
        let mut scanner = Scanner::new("test_illegal_character.sbl", source.as_bytes());

        match scanner.next() {
            Ok(_) => panic!("should not have gotten a token"),
            Err(diag) => {
                assert_eq!(diag.kind, DiagKind::SyntaxError);
                assert_eq!(diag.pos, Position::new(0, 1, 1, 1));
            }
        }
    }

    #[test]
    fn test_unterminated_string() {
        let source = "\"hi";
        let mut scanner = Scanner::new("test_unterminated_string.sbl", source.as_bytes());

        match scanner.next() {
            Ok(_) => panic!("should not have gotten a token"),
            Err(diag) => {
                assert_eq!(diag.kind, DiagKind::SyntaxError);
                assert_eq!(diag.pos, Position::new(2, 1, 3, 1));
            }
        }
    }
}
