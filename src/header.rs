use std::path::Path;

const HEADER_LINES: usize = 11;

const GENERIC_TEMPLATE: &str = "\
********************************************************************************
*                                                                              *
*                                                         :::      ::::::::    *
*    $FILENAME__________________________________        :+:      :+:    :+:    *
*                                                     +:+ +:+         +:+      *
*    By: $AUTHOR________________________________    +#+  +:+       +#+         *
*                                                 +#+#+#+#+#+   +#+            *
*    Created: $CREATEDAT_________ by $CREATEDBY_       #+#    #+#              *
*    Updated: $UPDATEDAT_________ by $UPDATEDBY_      ###   ########.fr        *
*                                                                              *
********************************************************************************

";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Upsert,
    UpdateOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommentStyle {
    Slash,
    Hash,
    Semicolon,
    Paren,
    Dash,
    Percent,
}

impl CommentStyle {
    pub fn parse(value: &str) -> Option<Self> {
        match normalize(value).as_str() {
            "slash" | "slashes" | "c" | "cpp" | "c++" => Some(Self::Slash),
            "hash" | "hashes" | "pound" | "python" => Some(Self::Hash),
            "semicolon" | "semicolons" | "ini" => Some(Self::Semicolon),
            "paren" | "parens" | "parentheses" | "ocaml" => Some(Self::Paren),
            "dash" | "dashes" | "lua" | "haskell" => Some(Self::Dash),
            "percent" | "percents" | "latex" | "tex" => Some(Self::Percent),
            _ => None,
        }
    }

    fn delimiters(self) -> (&'static str, &'static str) {
        match self {
            Self::Slash => ("/* ", " */"),
            Self::Hash => ("# ", " #"),
            Self::Semicolon => (";; ", " ;;"),
            Self::Paren => ("(* ", " *)"),
            Self::Dash => ("-- ", " --"),
            Self::Percent => ("%% ", " %%"),
        }
    }

    fn width(self) -> usize {
        let (left, _) = self.delimiters();
        left.len()
    }
}

#[derive(Debug)]
pub struct HeaderOptions<'a> {
    pub path: Option<&'a Path>,
    pub language: Option<&'a str>,
    pub style: Option<CommentStyle>,
    pub username: &'a str,
    pub email: &'a str,
    pub now: &'a str,
    pub mode: Mode,
}

#[derive(Debug, Eq, PartialEq)]
struct HeaderInfo {
    filename: String,
    author: String,
    created_at: String,
    created_by: String,
    updated_at: String,
    updated_by: String,
}

pub fn apply(input: &str, options: &HeaderOptions<'_>) -> String {
    let newline = detect_newline(input);
    let normalized = input.replace("\r\n", "\n");
    let Some(style) = resolve_style(options.style, options.language, options.path) else {
        return input.to_owned();
    };

    let existing = extract_header(&normalized);
    if existing.is_none() && options.mode == Mode::UpdateOnly {
        return input.to_owned();
    }

    let (existing_header, rest) = existing.unwrap_or(("", normalized.as_str()));
    let info = build_header_info(existing_header, options);
    let mut output = render_header(style, &info);
    output.push_str(rest);

    if newline == "\r\n" {
        output.replace('\n', "\r\n")
    } else {
        output
    }
}

fn build_header_info(existing_header: &str, options: &HeaderOptions<'_>) -> HeaderInfo {
    let filename = options
        .path
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .or_else(|| {
            field(existing_header, "FILENAME")
                .map(str::trim)
                .map(str::to_owned)
        })
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "unknown".to_owned());

    let created_at = field(existing_header, "CREATEDAT")
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(options.now)
        .to_owned();

    let created_by = field(existing_header, "CREATEDBY")
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(options.username)
        .to_owned();

    HeaderInfo {
        filename,
        author: format!("{} <{}>", options.username, options.email),
        created_at,
        created_by,
        updated_at: options.now.to_owned(),
        updated_by: options.username.to_owned(),
    }
}

fn render_header(style: CommentStyle, info: &HeaderInfo) -> String {
    let mut header = GENERIC_TEMPLATE.to_owned();
    set_field(&mut header, "FILENAME", &info.filename);
    set_field(&mut header, "AUTHOR", &info.author);
    set_field(&mut header, "CREATEDAT", &info.created_at);
    set_field(&mut header, "CREATEDBY", &info.created_by);
    set_field(&mut header, "UPDATEDAT", &info.updated_at);
    set_field(&mut header, "UPDATEDBY", &info.updated_by);
    apply_delimiters(&header, style)
}

fn apply_delimiters(header: &str, style: CommentStyle) -> String {
    let (left, right) = style.delimiters();
    let width = style.width();
    let mut rendered = String::with_capacity(header.len());

    for line in header.lines() {
        if line.is_empty() {
            rendered.push('\n');
            continue;
        }

        rendered.push_str(left);
        rendered.push_str(&line[width..line.len() - width]);
        rendered.push_str(right);
        rendered.push('\n');
    }

    rendered
}

fn extract_header(text: &str) -> Option<(&str, &str)> {
    let mut end = 0;
    let mut lines = Vec::with_capacity(HEADER_LINES);

    for _ in 0..HEADER_LINES {
        let next_newline = text[end..].find('\n')?;
        let line = &text[end..end + next_newline];
        if line.len() != 80 || !line.is_ascii() {
            return None;
        }
        lines.push(line);
        end += next_newline + 1;
    }

    if !looks_like_42_header(&lines) {
        return None;
    }

    if text[end..].starts_with('\n') {
        end += 1;
    }

    Some((&text[..end], &text[end..]))
}

fn looks_like_42_header(lines: &[&str]) -> bool {
    lines.len() == HEADER_LINES
        && lines[0].contains("********")
        && lines[2].contains(":::      ::::::::")
        && lines[7].contains("Created:")
        && lines[8].contains("Updated:")
        && lines[10].contains("********")
}

fn field<'a>(header: &'a str, name: &str) -> Option<&'a str> {
    if header.is_empty() {
        return None;
    }

    let (offset, width) = field_bounds(name)?;
    header.get(offset..offset + width)
}

fn set_field(header: &mut String, name: &str, value: &str) {
    let Some((offset, width)) = field_bounds(name) else {
        return;
    };
    header.replace_range(offset..offset + width, &fit(value, width));
}

fn field_bounds(name: &str) -> Option<(usize, usize)> {
    let marker = format!("${name}");
    let offset = GENERIC_TEMPLATE.find(&marker)?;
    let width = GENERIC_TEMPLATE[offset..]
        .bytes()
        .take_while(|byte| *byte == b'$' || byte.is_ascii_uppercase() || *byte == b'_')
        .count();
    Some((offset, width))
}

fn fit(value: &str, width: usize) -> String {
    let mut output: String = value
        .chars()
        .map(|ch| {
            if ch.is_ascii() && !ch.is_ascii_control() {
                ch
            } else {
                '?'
            }
        })
        .take(width)
        .collect();

    while output.len() < width {
        output.push(' ');
    }

    output
}

fn detect_newline(input: &str) -> &'static str {
    if input.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

fn resolve_style(
    explicit: Option<CommentStyle>,
    language: Option<&str>,
    path: Option<&Path>,
) -> Option<CommentStyle> {
    explicit
        .or_else(|| language.and_then(style_for_language))
        .or_else(|| path.and_then(style_for_path))
}

fn style_for_language(language: &str) -> Option<CommentStyle> {
    match normalize(language).as_str() {
        "c" | "cpp" | "c++" | "css" | "go" | "groovy" | "java" | "javascript" | "jsx"
        | "javascriptreact" | "less" | "objectivec" | "php" | "rust" | "scss" | "swift"
        | "typescript" | "tsx" | "typescriptreact" | "xsl" => Some(CommentStyle::Slash),
        "coffeescript" | "dockerfile" | "makefile" | "perl" | "perl6" | "plain text"
        | "plaintext" | "powershell" | "python" | "r" | "ruby" | "shell script" | "shellscript"
        | "sql" | "yaml" => Some(CommentStyle::Hash),
        "ini" => Some(CommentStyle::Semicolon),
        "fsharp" | "f#" | "ocaml" => Some(CommentStyle::Paren),
        "haskell" | "lua" => Some(CommentStyle::Dash),
        "latex" | "tex" => Some(CommentStyle::Percent),
        _ => None,
    }
}

fn style_for_path(path: &Path) -> Option<CommentStyle> {
    let file_name = path.file_name()?.to_str()?;
    match normalize(file_name).as_str() {
        "dockerfile" => return Some(CommentStyle::Hash),
        "makefile" | "gnumakefile" => return Some(CommentStyle::Hash),
        _ => {}
    }

    let extension = path.extension()?.to_str()?;
    match normalize(extension).as_str() {
        "c" | "cc" | "cpp" | "cxx" | "h" | "hh" | "hpp" | "hxx" | "css" | "go" | "groovy"
        | "java" | "js" | "jsx" | "less" | "m" | "mm" | "php" | "rs" | "scss" | "swift" | "ts"
        | "tsx" | "xsl" => Some(CommentStyle::Slash),
        "coffee" | "conf" | "dockerfile" | "mk" | "pl" | "pm" | "ps1" | "py" | "r" | "rb"
        | "sh" | "bash" | "zsh" | "sql" | "txt" | "yaml" | "yml" => Some(CommentStyle::Hash),
        "ini" => Some(CommentStyle::Semicolon),
        "fs" | "fsi" | "ml" | "mli" => Some(CommentStyle::Paren),
        "hs" | "lhs" | "lua" => Some(CommentStyle::Dash),
        "tex" | "sty" => Some(CommentStyle::Percent),
        _ => None,
    }
}

fn normalize(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .filter(|ch| !matches!(ch, '-' | '_' | ' '))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn options<'a>(path: &'a str, now: &'a str) -> HeaderOptions<'a> {
        HeaderOptions {
            path: Some(Path::new(path)),
            language: None,
            style: None,
            username: "tester",
            email: "tester@student.42.fr",
            now,
            mode: Mode::Upsert,
        }
    }

    #[test]
    fn inserts_c_header() {
        let output = apply(
            "int main(void) {}\n",
            &options("vscode-42header.c", "2013/11/18 13:37:42"),
        );

        assert!(output.starts_with(
            "/* ************************************************************************** */\n"
        ));
        assert!(output.contains(
            "/*   vscode-42header.c                                  :+:      :+:    :+:   */\n"
        ));
        assert!(output.contains(
            "/*   By: tester <tester@student.42.fr>              +#+  +:+       +#+        */\n"
        ));
        assert!(output.contains(
            "/*   Created: 2013/11/18 13:37:42 by tester            #+#    #+#             */\n"
        ));
        assert!(output.ends_with("\nint main(void) {}\n"));
    }

    #[test]
    fn updates_existing_header_preserving_created_fields() {
        let first = apply(
            "int main(void) {}\n",
            &options("old.c", "2013/11/18 13:37:42"),
        );
        let second = apply(&first, &options("new.c", "2016/09/18 13:11:04"));

        assert!(second.contains(
            "/*   new.c                                              :+:      :+:    :+:   */\n"
        ));
        assert!(second.contains(
            "/*   Created: 2013/11/18 13:37:42 by tester            #+#    #+#             */\n"
        ));
        assert!(second.contains(
            "/*   Updated: 2016/09/18 13:11:04 by tester           ###   ########.fr       */\n"
        ));
        assert_eq!(second.matches("Created:").count(), 1);
    }

    #[test]
    fn leaves_unsupported_files_unchanged() {
        let input = "<html></html>\n";
        let output = apply(input, &options("index.html", "2013/11/18 13:37:42"));

        assert_eq!(output, input);
    }

    #[test]
    fn supports_update_only_mode() {
        let mut opts = options("main.c", "2013/11/18 13:37:42");
        opts.mode = Mode::UpdateOnly;

        assert_eq!(apply("int main(void) {}\n", &opts), "int main(void) {}\n");
    }

    #[test]
    fn preserves_crlf_line_endings() {
        let output = apply(
            "int main(void) {}\r\n",
            &options("main.c", "2013/11/18 13:37:42"),
        );

        assert!(output.contains("\r\n"));
        assert!(!output.contains("*/\n"));
    }

    #[test]
    fn can_force_comment_style() {
        let mut opts = options("unknown.ext", "2013/11/18 13:37:42");
        opts.style = Some(CommentStyle::Hash);
        let output = apply("body\n", &opts);

        assert!(output.starts_with(
            "# **************************************************************************** #\n"
        ));
    }
}
