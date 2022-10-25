use crate::lex;

pub fn parse(lexer: &mut lex::Lex){
    statements(lexer);
}

fn statements(lexer: &mut lex::Lex) {
    loop{
        match lexer.match_next(lex::Token::EOF) {
            true => {
                // file ended
                println!("EOF reached");
                break;
            },
            false => {
                let t = expression(lexer);
                println!("{};", t);
                free(t);
            }
        }
    }
}

fn expression(lexer: &mut lex::Lex) -> char {
    // get the first term
    let t = term(lexer);

    if lexer.match_next(lex::Token::SEMI) {
        lexer.next();
        return t
    }

    while lexer.match_next(lex::Token::ADD) {
        lexer.next(); // remove the add

        // get the next term
        let t2 = term(lexer);
        println!("{} += {};", t, t2);
        free(t2);

        // break if next is semicolon
        if lexer.match_next(lex::Token::SEMI){
            lexer.next();
            break;
        }
    }

    t
}


fn term(lexer: &mut lex::Lex) -> char {
    let fac = factor(lexer);

    while lexer.match_next(lex::Token::MUL) {
        lexer.next();

        let fac2 = factor(lexer);
        println!("{} *= {};", fac, fac2);

        free(fac2);
    }

    fac
}

fn factor(lexer: &mut lex::Lex) -> char {
    match lexer.next() {
        lex::Token::NUM_OR_ID => {
            let num_or_id = lexer.cur_lexeme();
            let s = getTemp(num_or_id);

            println!("{} = {};", s, num_or_id);

            return s;
        },
        lex::Token::LP => {
            let t = expression(lexer);

            if lexer.match_next(lex::Token::RP){
                lexer.next();
            } else {
                panic!("Right paranthese is missing")
            }

           return  t;
        }
        x => panic!("Error: {:?}", x)
    }
}


static TEMPS: [char; 9] = ['X', 'Y', 'Z', 'W', 'U', 'V', 'R', 'S', 'T'];

use std::sync::atomic::{AtomicUsize, Ordering};

static POS: AtomicUsize = AtomicUsize::new(0);

fn free(_s: char) {
    POS.fetch_sub(1, Ordering::SeqCst);
}

fn getTemp(_s: &str) -> char {
    // println!("s.{}", s);
    let r = TEMPS[POS.load(Ordering::SeqCst)];
    POS.fetch_add(1, Ordering::SeqCst);
    r
}
