mod header;

use chrono::Local;
use header::{apply, CommentStyle, HeaderOptions, Mode};
use std::{
    env, fs,
    io::{self, Read, Write},
    path::PathBuf,
    process,
};

#[derive(Debug)]
struct Cli {
    path: Option<PathBuf>,
    language: Option<String>,
    style: Option<CommentStyle>,
    username: Option<String>,
    email: Option<String>,
    mode: Mode,
    write: bool,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("zed-42header: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let cli = parse_args(env::args().skip(1))?;
    let username = cli.username.unwrap_or_else(default_username);
    let email = cli.email.unwrap_or_else(|| default_email(&username));
    let now = Local::now().format("%Y/%m/%d %H:%M:%S").to_string();

    if cli.write {
        let path = cli
            .path
            .as_ref()
            .ok_or_else(|| "--write requires --stdin-filepath or --file".to_owned())?;
        let input = fs::read_to_string(path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
        let output = apply(
            &input,
            &HeaderOptions {
                path: Some(path.as_path()),
                language: cli.language.as_deref(),
                style: cli.style,
                username: &username,
                email: &email,
                now: &now,
                mode: cli.mode,
            },
        );

        if output != input {
            fs::write(path, output)
                .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
        }
        return Ok(());
    }

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("failed to read stdin: {error}"))?;

    let output = apply(
        &input,
        &HeaderOptions {
            path: cli.path.as_deref(),
            language: cli.language.as_deref(),
            style: cli.style,
            username: &username,
            email: &email,
            now: &now,
            mode: cli.mode,
        },
    );

    io::stdout()
        .write_all(output.as_bytes())
        .map_err(|error| format!("failed to write stdout: {error}"))?;

    Ok(())
}

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Cli, String> {
    let mut cli = Cli {
        path: None,
        language: None,
        style: None,
        username: None,
        email: None,
        mode: Mode::Upsert,
        write: false,
    };

    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                process::exit(0);
            }
            "--stdin-filepath" | "--file" | "--path" => {
                cli.path = Some(PathBuf::from(value_for(&arg, &mut args)?));
            }
            "--language" => {
                cli.language = Some(value_for(&arg, &mut args)?);
            }
            "--comment-style" | "--style" => {
                let value = value_for(&arg, &mut args)?;
                cli.style = CommentStyle::parse(&value)
                    .ok_or_else(|| format!("unknown comment style: {value}"))?
                    .into();
            }
            "--user" | "--username" => {
                cli.username = Some(value_for(&arg, &mut args)?);
            }
            "--email" => {
                cli.email = Some(value_for(&arg, &mut args)?);
            }
            "--update-only" | "--no-insert" => {
                cli.mode = Mode::UpdateOnly;
            }
            "--write" => {
                cli.write = true;
            }
            _ if arg.starts_with("--stdin-filepath=") => {
                cli.path = Some(PathBuf::from(arg_value(&arg)));
            }
            _ if arg.starts_with("--file=") || arg.starts_with("--path=") => {
                cli.path = Some(PathBuf::from(arg_value(&arg)));
            }
            _ if arg.starts_with("--language=") => {
                cli.language = Some(arg_value(&arg));
            }
            _ if arg.starts_with("--comment-style=") || arg.starts_with("--style=") => {
                let value = arg_value(&arg);
                cli.style = CommentStyle::parse(&value)
                    .ok_or_else(|| format!("unknown comment style: {value}"))?
                    .into();
            }
            _ if arg.starts_with("--user=") || arg.starts_with("--username=") => {
                cli.username = Some(arg_value(&arg));
            }
            _ if arg.starts_with("--email=") => {
                cli.email = Some(arg_value(&arg));
            }
            _ => return Err(format!("unknown argument: {arg}")),
        }
    }

    Ok(cli)
}

fn value_for(flag: &str, args: &mut impl Iterator<Item = String>) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn arg_value(arg: &str) -> String {
    arg.split_once('=')
        .map(|(_, value)| value.to_owned())
        .unwrap_or_default()
}

fn default_username() -> String {
    env::var("FT_HEADER_USERNAME")
        .or_else(|_| env::var("42HEADER_USERNAME"))
        .or_else(|_| env::var("USER"))
        .or_else(|_| env::var("LOGNAME"))
        .unwrap_or_else(|_| "marvin".to_owned())
}

fn default_email(username: &str) -> String {
    env::var("FT_HEADER_EMAIL")
        .or_else(|_| env::var("42HEADER_EMAIL"))
        .unwrap_or_else(|_| format!("{username}@student.42.fr"))
}

fn print_help() {
    println!(
        "\
zed-42header

Reads source text from stdin and writes the same text with a 42 header to stdout.

USAGE:
  zed-42header --stdin-filepath <path> [options] < input > output
  zed-42header --write --stdin-filepath <path> [options]

OPTIONS:
  --stdin-filepath <path>   Path used for filename and comment-style detection
  --language <name>         Zed language name, such as C, Python, or Shell Script
  --comment-style <style>   Force slash, hash, semicolon, paren, dash, or percent
  --user <name>             Header username (default: FT_HEADER_USERNAME or USER)
  --email <email>           Header email (default: FT_HEADER_EMAIL or user@student.42.fr)
  --update-only             Update an existing header but do not insert a new one
  --write                   Read and update --stdin-filepath directly
"
    );
}
