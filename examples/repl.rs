use std::io;
use std::sync::Arc;

use linefeed::{Interface, ReadResult};

use marin::Marin;

fn main() -> io::Result<()> {
    let interface = Arc::new(Interface::new(env!("CARGO_PKG_NAME"))?);

    println!("Enter \"help\" for examples.");
    println!("Press Ctrl-D or enter \"exit\" to exit.");
    println!("(Parsing errors are not handled right now, nor is output formatted nicely)");
    println!();

    interface.set_prompt("marin> ")?;

    while let ReadResult::Input(line) = interface.read_line()? {
        let (cmd, _) = split_first_word(&line);

        match cmd {
            "h" | "help" => {
                println!("Examples:");
                println!("  Positonal Arguments");
                println!("    1 2 3");
                println!("    arg1 arg2 arg3");
                println!("  Keyword Arguments");
                println!("    kw: 1");
                println!("    keywordarg: True");
                println!("  Flags");
                println!("    -flag1 -flag2");
                println!("  Quoted Strings");
                println!("    kw: \"string with spaces\"");
                println!("    \"positional argument with spaces\"");
                println!("  Ranges");
                println!("    range: 1..10");
                println!("    1..10");
                println!("    ..10");
                println!("    ids: -10..20");
                println!("  Lists");
                println!("    vals: [\"val1\", \"val2\"]");
                println!("    vals: [1, 2, 3]");
                println!("    [1,2,3]");
                println!();
            }
            "q" | "exit" => break,
            _ => {
                if !line.trim().is_empty() {
                    let result = Marin::parse(&line).unwrap();
                    println!("{:#?}", result);
                    println!()
                }
            }
        }
    }

    Ok(())
}

fn split_first_word(s: &str) -> (&str, &str) {
    let s = s.trim();

    match s.find(|ch: char| ch.is_whitespace()) {
        Some(pos) => (&s[..pos], s[pos..].trim_start()),
        None => (s, ""),
    }
}
