use clap::Parser;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[clap(author, version, about = "List directories recursively, usually paired with fzf for fuzzy directory navigation", long_about = None)]
struct Options {
    #[clap(short, long, help = "List hidden directories", value_parser, default_value_t = false)]
    list_hidden: bool,
    #[clap(short, long, help = "Follow symlinks. WARNING: may recurse infinitely when used with -i", value_parser, default_value_t = false)]
    follow_symlinks: bool,
    #[clap(short, long, help = "Ignore all errors. NOTE: not recommended, but may be useful when searching directories with mixed permissions", value_parser, default_value_t = false)]
    ignore_errors: bool,
    #[clap(value_parser, help = "The directory to be listed (defaults to the current directory)")]
    dir: Option<String>,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|e| e.starts_with("."))
         .unwrap_or(false)
}

fn can_be_shown(options: &Options, entry: &DirEntry) -> bool {
    if options.list_hidden {
        return true;
    } else {
        return !is_hidden(entry);
    }
}

fn main() {
    let options = Options::parse();

    let mut result = String::new();
    let dir = options.dir.clone().unwrap_or(".".to_string());
    let mut iter = WalkDir::new(dir)
                .follow_links(options.follow_symlinks)
                .into_iter();

    // skip first entry, usually the '.' directory
    iter.next();

    loop {
        let e = match iter.next() {
            Some(Ok(e)) => e,
            Some(Err(err)) => {
                if options.ignore_errors {
                    continue;
                } else {
                    panic!("Error: {}", err);
                }
            },
            None => break,
        };

        if e.file_type().is_dir() {
            if can_be_shown(&options, &e) {
                result.push_str(&e.path().display().to_string());
            } else {
                iter.skip_current_dir();
                continue;
            }
        } else {
            continue;
        }

        result.push('\n');
    }

    print!("{}", result);
}
