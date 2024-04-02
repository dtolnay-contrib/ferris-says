use regex::Regex;
use std::io::{Result, Write};
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
pub fn say<W>(input: &str, max_width: usize, mut writer: W) -> Result<()>
where
    W: Write,
{
    // Final output is stored here
    let mut write_buffer = Vec::new();

    // Pre process to merge continuous whitespaces into one space character
    let input = merge_white_spaces(input);

    // Let textwrap work its magic
    let wrapped = fill(input.as_str(), max_width);

    let lines: Vec<&str> = wrapped.lines().collect();

    let line_count = lines.len();
    let actual_width = longest_line(&lines);

    // top box border
    write_buffer.push(b' ');
    for _ in 0..(actual_width + 2) {
        write_buffer.push(b'_');
    }
    write_buffer.push(b'\n');

    // inner message
    for (i, line) in lines.into_iter().enumerate() {
        if line_count == 1 {
            write_buffer.extend_from_slice(b"< ");
        } else if i == 0 {
            write_buffer.extend_from_slice(b"/ ");
        } else if i == line_count - 1 {
            write_buffer.extend_from_slice(b"\\ ");
        } else {
            write_buffer.extend_from_slice(b"| ");
        }

        let line_len = UnicodeWidthStr::width(line);
        write_buffer.extend_from_slice(line.as_bytes());
        for _ in line_len..actual_width {
            write_buffer.push(b' ');
        }

        if line_count == 1 {
            write_buffer.extend_from_slice(b" >\n");
        } else if i == 0 {
            write_buffer.extend_from_slice(b" \\\n");
        } else if i == line_count - 1 {
            write_buffer.extend_from_slice(b" /\n");
        } else {
            write_buffer.extend_from_slice(b" |\n");
        }
    }

    // bottom box border
    write_buffer.push(b' ');
    for _ in 0..(actual_width + 2) {
        write_buffer.push(b'-');
    }

    // mascot
    write_buffer.extend_from_slice(MASCOT);

    writer.write_all(&write_buffer)
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
