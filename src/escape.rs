use pulldown_cmark::{Options, Parser};

#[inline]
pub fn escape(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for c in input.chars() {
        match c {
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&apos;"),
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            o => result.push(o),
        }
    }
    result
}

pub fn md_to_text(input: &str) -> String {
    use pulldown_cmark::Event::*;
    Parser::new_ext(input, Options::ENABLE_TABLES)
        .filter_map(|ev| match ev {
            Text(val) => Some(val),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn md_to_html(input: &str) -> String {
    use pulldown_cmark::Event::*;

    let filter_html =
        Parser::new_ext(input, Options::ENABLE_TABLES).filter(|ev| !matches!(ev, Html(_)));

    let mut html_buf = String::new();

    pulldown_cmark::html::push_html(&mut html_buf, filter_html);

    html_buf
}

pub fn truncate(input: &str, len: usize) -> String {
    if input.len() <= len {
        return input.to_string();
    }

    let mut end_idx = len + 1;

    while !input.is_char_boundary(end_idx) {
        end_idx -= 1;
    }

    let slice = &input[0..end_idx];

    let mut end_idx = len;

    if let Some(val) = slice.rfind(char::is_whitespace) {
        end_idx = val;
    }

    format!("{}...", &input[0..end_idx])
}
