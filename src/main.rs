use std::env;
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn path_str(path: PathBuf) -> String {
    path.into_os_string().into_string().unwrap()
}

const ICON: &str = "🐈";
const VIEW: usize = 2;

fn main() {
    let path = path_str(env::current_dir().unwrap());
    let segments: Vec<_> = path.split('/').collect();
    let view = segments.len().saturating_sub(VIEW);
    let (abbr, local) = (segments[..view].iter(), segments[view..].iter());
    let user = &env::var("USER").unwrap();
    let mut result = Vec::new();
    for &segment in abbr {
        if segment == "" {
            continue;
        } else if segment == user {
            result.push(ICON);
        } else {
            result.push(&segment[..1]);
        }
    }
    for &segment in local {
        if segment == user {
            result.push(ICON);
        } else {
            result.push(segment);
        }
    }
    print!("{}", result[0]);
    for segment in &result[1..] {
        print!("/{segment}");
    }
    print!(": ");
    io::stdout().flush().unwrap();
}
