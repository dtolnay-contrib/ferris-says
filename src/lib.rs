use regex::Regex;
use std::io::{BufWriter, Result, Write};
use textwrap::fill;
use unicode_width::UnicodeWidthStr;

const MASCOT: &[u8] = if cfg!(feature = "clippy") {
    br#"
        \
         \
            __
           /  \
           |  |
           @  @
           |  |
           || |/
           || ||
           |\_/|
           \___/
"#
} else {
    br#"
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#
};

/// Print out Ferris saying something.
///
/// `input` is a string slice that you want to be written out to somewhere
///
/// `max_width` is the maximum width of a line of text before it is wrapped
///
/// `writer` is anywhere that can be written to using the Writer trait like
/// STDOUT or STDERR
///
/// # Example
///
/// The following bit of code will write the byte string to STDOUT
///
/// ```rust
/// use ferris_says::say;
/// use std::io::{stdout, BufWriter};
///
/// let stdout = stdout();
/// let out = "Hello fellow Rustaceans!";
/// let width = 24;
///
/// let writer = BufWriter::new(stdout.lock());
/// say(out, width, writer).unwrap();
/// ```
///
/// This will print out:
///
/// ```plain
///  __________________________
/// < Hello fellow Rustaceans! >
///  --------------------------
///         \
///          \
///             _~^~^~_
///         \) /  o o  \ (/
///           '_   -   _'
///           / '-----' \
/// ```
pub fn say<W>(input: &str, max_width: usize, writer: W) -> Result<()>
where
    W: Write,
{
    let mut writer = BufWriter::new(writer);

    // Pre process to merge continuous whitespaces into one space character
    let input = merge_white_spaces(input);

    // Let textwrap work its magic
    let wrapped = fill(input.as_str(), max_width);

    let lines: Vec<&str> = wrapped.lines().collect();

    let line_count = lines.len();
    let actual_width = longest_line(&lines);

    // top box border
    writer.write_all(b" ")?;
    for _ in 0..(actual_width + 2) {
        writer.write_all(b"_")?;
    }
    writer.write_all(b"\n")?;

    // inner message
    for (i, line) in lines.into_iter().enumerate() {
        if line_count == 1 {
            writer.write_all(b"< ")?;
        } else if i == 0 {
            writer.write_all(b"/ ")?;
        } else if i == line_count - 1 {
            writer.write_all(b"\\ ")?;
        } else {
            writer.write_all(b"| ")?;
        }

        let line_len = UnicodeWidthStr::width(line);
        writer.write_all(line.as_bytes())?;
        for _ in line_len..actual_width {
            writer.write_all(b" ")?;
        }

        if line_count == 1 {
            writer.write_all(b" >\n")?;
        } else if i == 0 {
            writer.write_all(b" \\\n")?;
        } else if i == line_count - 1 {
            writer.write_all(b" /\n")?;
        } else {
            writer.write_all(b" |\n")?;
        }
    }

    // bottom box border
    writer.write_all(b" ")?;
    for _ in 0..(actual_width + 2) {
        writer.write_all(b"-")?;
    }

    // mascot
    writer.write_all(MASCOT)?;

    Ok(())
}

fn longest_line(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| UnicodeWidthStr::width(*line))
        .max()
        .unwrap_or(0)
}

/// Merge continues white spaces into one space character while preserving newline characters.
fn merge_white_spaces(input: &str) -> String {
    let re = Regex::new(r"([^\S\r\n])+").unwrap();
    re.replace_all(input, " ").to_string()
}
