use rinterpreter::repl;
use std::io;
use whoami;

fn main() -> Result<(), std::io::Error> {
    let user = whoami::username();
    println!("Hello {}!, Welcome to the Rinterpreter REPL!", user);
    let mut stdin = io::BufReader::new(io::stdin());
    let mut stdout = io::BufWriter::new(io::stdout());

    repl::start(&mut stdin, &mut stdout)?;
    Ok(())
}
