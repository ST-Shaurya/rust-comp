use crate::lex;

pub fn parse(lexer: &mut lex::Lex){


    statements(lexer);
}

fn statements(lexer: &mut lex::Lex) {
    loop{
        match lexer.match_next(lex::_EOF) {
            true => {
                // file ended
                println!("EOF reached");
                break;
            },
            false => {
                expression(lexer);
            }
        }
    }
}

fn expression(lexer: &mut lex::Lex) {
    // get the first term
    term(lexer);

    while lexer.match_next(lex::_OP_PLUS) {
        println!("found +");
        lexer.next(); // remove the add

        // get the next term
        term(lexer);

        // break if next is semicolon
        if lexer.match_next(lex::_SEMI){
            println!("found ;");
            lexer.next();
            break;
        }
    }
}


fn term(lexer: &mut lex::Lex) {
    factor(lexer);

    while lexer.match_next(lex::_OP_MUL) {
        println!("found *");
        factor(lexer);
    }
}

fn factor(lexer: &mut lex::Lex) {
    match lexer.next() {
        lex::_NUM_OR_ID => {
            println!("Found Num or id {:?}", lexer.cur_lexeme())
        },
        lex::_LP => {
            println!("found (");
            expression(lexer);
            if lexer.match_next(lex::_RP){
                println!("found )");
                lexer.next();
            } else {
                panic!("Right paranthese is missing")
            }
        }
        _ => return
    }
}
