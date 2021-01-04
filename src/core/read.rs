use rustyline::{
    error::ReadlineError,
    Editor,
};

/*
 * loop readline.
 */
pub fn readline() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("(main)> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
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
