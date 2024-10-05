use std::sync::LazyLock;

use genanki_rs::{basic_model, Deck, Note};
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
    let title_regex = Regex::new(r"^#\s+").unwrap();
    let options = Options::gfm();
    for i in std::fs::read_dir(config.input_dir).unwrap() {
        let path = i.unwrap().path();
        println!(
            "Reading file: {}",
            path.display()
        );
        let file_content = std::fs::read_to_string(path).unwrap();
        let (title, body) = file_content.split_once('\n').unwrap();
        let title = markdown::to_html(&title_regex.replace(title, ""));
        let mut body = body.to_string();
        let tags: Vec<&str> = TAG_REGEX
            .captures_iter(&file_content)
            .map(|x| {
                let i_want = x.name("tag").unwrap();
                i_want.as_str()
            })
            .collect();
        body = remove_links(body);
        body = remove_tags(body);
        body = replace_math(body);
        body = markdown::to_html_with_options(&body, &options).unwrap();
        let mut note =
            Note::new(basic_model(), vec![&title, &body]).expect("Couldn't generate note");
        note = note.tags(tags);
        deck.add_note(note);
    }
    deck.write_to_file(&format!("{}/{}.apkg", config.output_dir, config.deck_name))
        .unwrap();
}
