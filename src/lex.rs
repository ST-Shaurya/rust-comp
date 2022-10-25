#[derive(Debug, PartialEq)]
#[allow(dead_code)]
#[repr(u8)]
pub enum Token {
    EOF = 0,
    SEMI = 1,

    ADD = 2,
    SUB = 3,
    MUL = 4,
    DIV = 5,
    POW = 6,

    LP = 7,
    RP = 8,

    NUM_OR_ID = 9,
}

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
    pub fn next(&mut self) -> Token{

        // set cur to start of next lexeme
        self.cur += self.len;

        // set len to 0 as we dont know the len of lexeme right now
        self.len = 0usize;

        // if it has reached the end of file, return EOF
        if self.fstr.chars().count() == self.cur {
            return Token::EOF;
        }

        let c = self.fstr.chars().nth(self.cur); // get the next char
        self.len += 1; // inc the len since we took out one char

        // matches the char
        let token = match c {
            Some(';') => Token::SEMI,
            Some('+') => Token::ADD,
            Some('-') => Token::SUB,
            Some('/') => Token::DIV,
            Some('^') => Token::POW,
            Some('*') => Token::MUL,
            Some('(') => Token::LP,
            Some(')') => Token::RP,
            Some(c) if c.is_ascii_alphanumeric() => {
                // loop and get the entire lexeme
                loop {
                    match self.fstr.chars().nth(self.cur + self.len) {
                        // keep inc len as long as we get a alpha numeric string
                        Some(c) if c.is_ascii_alphanumeric() => self.len += 1,
                        // otherwise break, len would now be the size of lexeme
                        _ => break,
                    };
                };
                Token::NUM_OR_ID
            },
            Some('\n') => {
                // maintaining linenumber for compile time error detection
                self.lineno += 1;
                self.next()
            },
            Some(' ' | '\t') => {
                // ignores tab and white space characters
                self.next()
            },
            _ => panic!("Syntax error: Unknown token at line no: {}", self.lineno)
        };

        token
    }

    pub fn lookahead(&mut self) -> Token {
        // save current state
        let lineno = self.lineno;
        let cur = self.cur;
        let len = self.len;

        // get next state
        let res = self.next();

        // reset last state
        self.lineno = lineno;
        self.cur = cur;
        self.len = len;

        res
    }

    /// returns true if next token matches the given token
    pub fn match_next(&mut self, token: Token) -> bool {
        let res = self.lookahead();

        // return result of cmp
        res == token
    }

    pub fn legal_lookahead(&mut self, tokens: Vec<Token>) -> bool {
        let lookahead = self.lookahead();

        let mut flag = false;
        for token in &tokens {
            if *token == lookahead {
                flag = true;
                break;
            }
        }

        if !flag {
            panic!("Syntax Error at line no: {}\n\tExpexted: {:?}\n\tGot: {:?}",
                   self.lineno,
                   tokens,
                   lookahead
            );
        }

        // return result
        flag
    }

}
