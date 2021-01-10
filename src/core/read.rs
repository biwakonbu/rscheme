use rustyline::{
    error::ReadlineError,
    Editor,
};

use crate::core::{
    eval::eval,
    token::AST,
    token::tokenize
};

/*
 * loop readline.
 */
pub fn prompt() {
    let mut rl = Editor::<()>::new();
    let mut ast = AST::new();

    loop {
        let readline = rl.readline("(main)> ");
        match readline {
            Ok(line) => {
                ast = tokenize(line.as_str());
                rl.add_history_entry(line.as_str());
                eval(ast.clone());
            },
            Err(ReadlineError::Interrupted) => {
                continue
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
