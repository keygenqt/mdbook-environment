use mdbook::BookItem;
use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use std::env;
use std::io;
use std::process::{Command, Stdio};

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return;
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    if let Err(e) = handle_preprocessing() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

struct MdBookEnvironment;

impl Preprocessor for MdBookEnvironment {
    fn name(&self) -> &str {
        "environment"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        if let Some(config) = _ctx.config.get_preprocessor("environment") {
            for (key, value) in config.into_iter() {
                if key != "command" {
                    match value.as_str() {
                        Some(value) => {
                            let set_value: Option<String> = if value.starts_with("$(") & value.ends_with(")") {
                                let command = &value[2..value.len() - 1];
                                // match Command::new(program).args(&command[1..command.len()]).stdout(Stdio::piped()).output() {
                                match Command::new("sh").args(["-c", command]).stdout(Stdio::piped()).output() {
                                    Ok(output) => {
                                        if output.status.success() {
                                            if let Ok(stdout) = String::from_utf8(output.stdout) {
                                                Some(stdout.trim().to_string())
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Err(_) => {
                                        None
                                    }
                                }
                            } else {
                                Some(value.to_string())
                            };
                            if let Some(value) = set_value {
                                unsafe {
                                    env::set_var(key, value);
                                }
                            } else {
                                eprintln!("Error init key: '{key}'!");
                            }
                        }
                        None => {
                            eprintln!("only string value");
                        }
                    }
                }
            }
        }
        book.for_each_mut(|item| {
            let BookItem::Chapter(ch) = item else {
                return;
            };
            if ch.is_draft_chapter() {
                return;
            }
            match replace_variables(ch) {
                Ok(s) => ch.content = s,
                Err(e) => eprintln!("failed to process chapter: {e:?}"),
            }
        });
        Ok(book)
    }
}

// ANCHOR: replace variables
fn replace_variables(chapter: &mut Chapter) -> Result<String, Error> {
    let mut content = chapter.content.clone();
    if !content.contains("{{") {
        return Ok(content);
    }
    let keys: Vec<Option<&str>> = chapter
        .content
        .split("{{")
        .filter(|e| e.contains("}}"))
        .map(|e| e.split("}}").next())
        .collect();
    for opt_key in keys {
        if let Some(key) = opt_key {
            match env::var(key.trim()) {
                Ok(value) => {
                    content = content.replace(&format!("{}{key}{}", "{{", "}}"), &value);
                }
                Err(_) => {
                    eprintln!("Not init key: {}{key}{}'.", "{{", "}}");
                }
            };
        }
    }
    Ok(content)
}
// ANCHOR_END: replace variables

pub fn handle_preprocessing() -> Result<(), Error> {
    let pre = MdBookEnvironment;
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}
