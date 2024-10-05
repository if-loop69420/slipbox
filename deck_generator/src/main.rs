use std::str::FromStr;
use std::sync::LazyLock;

use genanki_rs::{basic_model, Deck};
use markdown::Options;
use regex::{Captures, Regex};

const TAG_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"#(?<tag>[^#\s]+)").unwrap());
const LINK_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(\[{2}.*?\]{2})"#).unwrap());
const MATH_REGEX_DOUBLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(\${2})(?<content>.*?)(\${2})"#).unwrap());
const MATH_REGEX_SINGLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(\$)(?<content>.*?)(\$)"#).unwrap());
const MATH_NUMBER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\\(?<symbol>[A-Z]+)"#).unwrap());
const TITLE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^#\s+").unwrap());

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    input_dir: String,
    output_dir: String,
    deck_id: i64,
    deck_name: String,
    deck_description: String,
}

fn read_config() -> Config {
    let config_file = std::fs::read_to_string("./config.json").unwrap();
    serde_json::from_str(&config_file).unwrap()
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Note {
    title: String,
    body: String,
    tags: Vec<String>,
}

impl FromStr for Note {
    type Err = ();

    fn from_str(markdown: &str) -> Result<Self, Self::Err> {
        let (title, body) = markdown.split_once('\n').unwrap();
        let title = markdown::to_html(&TITLE_REGEX.replace(title, ""));
        let mut body = body.to_string();
        let tags = TAG_REGEX
            .captures_iter(&markdown)
            .map(|x| {
                let i_want = x.name("tag").unwrap();
                i_want.as_str().to_string()
            })
            .collect();
        body = remove_links(body);
        body = remove_tags(body);
        body = replace_math(body);
        let options = Options::gfm();
        body = markdown::to_html_with_options(&body, &options).unwrap();
        Ok(Self {
            title,
            body,
            tags
        })
    }
}

impl Into<genanki_rs::Note> for Note {
    fn into(self) -> genanki_rs::Note {
        genanki_rs::Note::new(basic_model(), vec![&self.title, &self.body])
            .expect("Couldn't generate note")
            .tags(self.tags)
    }
}

fn remove_links(input: String) -> String {
    LINK_REGEX.replace_all(&input, "").into_owned()
}

fn replace_math(input: String) -> String {
    let replaced_double = MATH_REGEX_DOUBLE
        .replace_all(&input, |caps: &Captures| {
            let content = caps.name("content").unwrap();
            format!("\\[ {} \\]", content.as_str())
        })
        .into_owned();
    let single_replaced = MATH_REGEX_SINGLE
        .replace_all(&replaced_double, |caps: &Captures| {
            let content = caps.name("content").unwrap();
            format!("\\( {} \\)", content.as_str())
        })
        .into_owned();
    let number_symbol_replaced = MATH_NUMBER.replace_all(&single_replaced, "\\mathbb{$symbol}");
    number_symbol_replaced.to_string()
}

fn remove_tags(input: String) -> String {
    TAG_REGEX.replace_all(&input, "").into_owned()
}

// Read config.
// Go through all input files. Get the title with regex. Get the rest of the content with a regex. Generate new anki note.
// Put note into deck.
// Save deck to file
fn main() {
    let config = read_config();
    let mut deck = Deck::new(config.deck_id, &config.deck_name, &config.deck_description);
    for i in std::fs::read_dir(config.input_dir).unwrap() {
        let path = i.unwrap().path();
        println!(
            "Reading file: {}",
            path.clone().into_os_string().into_string().unwrap()
        );
        let file_content = std::fs::read_to_string(path).unwrap();
        let note: Note = file_content.parse().unwrap();
        deck.add_note(note.into());
    }
    deck.write_to_file(&format!("{}/{}.apkg", config.output_dir, config.deck_name))
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple_note() {
        let data = include_str!("test_data/20240910102356.md");
        // let note = parse_markdown(&data);
    }
}
