pub const _EOF: u8 = 0;
pub const _SEMI: u8 = 1;
pub const _OP_PLUS: u8 = 2;
pub const _OP_MUL: u8 = 3;
pub const _NUM_OR_ID: u8 = 4;
pub const _LP: u8 = 5;
pub const _RP: u8 = 6;

pub struct Lex {
    pub cur: usize,
    pub len: usize,
    pub lineno: usize,
    pub fstr: String
}

impl Lex {
    /// returns lexeme at current location
    pub fn cur_lexeme(&self) -> &str {
        &self.fstr[self.cur..(self.cur + self.len)]
    }

    /// returns the next token
    pub fn next(&mut self) -> u8{

        self.cur += self.len;
        self.len = 0usize;

        // if it has reached the end of file, return EOF
        if self.fstr.chars().count() == self.cur {
            return _EOF;
        }

        let c = self.fstr.chars().nth(self.cur); // get the next char
        self.len += 1; // inc the len since we took out one char

        // matches the char
        let token = match c {
            Some(';') => _SEMI,
            Some('+') => _OP_PLUS,
            Some('*') => _OP_MUL,
            Some('(') => _LP,
            Some(')') => _RP,
            Some('0'..='9') => {
                // loop to increase len up to the length of the number
                loop {
                    match self.fstr.chars().nth(self.cur + self.len) {
                        Some('0'..='9') => self.len += 1,
                        _ => break,
                    };
                };
                _NUM_OR_ID
            },
            Some('\n') => {
                self.lineno += 1;
                self.next()
            },
            _ => panic!("Unrecognized character")
        };

        token
    }

    /// returns true if next token matches the given token
    pub fn match_next(&mut self, _token: u8) -> bool {
        let lineno = self.lineno;
        let cur = self.cur;
        let len = self.len;

        let res = self.next();

        self.lineno = lineno;
        self.cur = cur;
        self.len = len;

        res == _token
    }

}
