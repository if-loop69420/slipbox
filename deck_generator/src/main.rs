use std::sync::LazyLock;
use std::{fs::FileType, str::FromStr};

use genanki_rs::{basic_model, Deck};
use markdown::Options;
use regex::{Captures, Regex};

const TAG_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"#(?<tag>[^#\s]+)").unwrap());
const LINK_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(\[{2}.*?\]{2})"#).unwrap());
const MATH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(:?(\${2}\s*)(?<content_double>.*?)(\${2}))|(:?(\$\s*)(?<content_single>.*?)(\$))"
    ).unwrap()
});
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
        Ok(Self { title, body, tags })
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
    MATH_REGEX
        .replace_all(&input, |caps: &Captures| {
            if let Some(content) = caps.name("content_double") {
                format!("[$]{}[/$]", content.as_str())
            } else if let Some(content) = caps.name("content_single") {
                format!("[$]{}[/$])", content.as_str())
            // } else if let Some(content) = caps.name("symbol") {
                // format!("\\\\mathbb{{ {} }}", content.as_str())
            } else {
                unreachable!()
            }
        })
        .into_owned()
        .to_string()
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
        let file = i.unwrap();
        let file_name: String = file.file_name().into_string().unwrap();
        let path = file.path();
        println!("Reading file: {}", path.display());
        let file_content = std::fs::read_to_string(path).unwrap();
        if file_content.is_empty()
            || file_content.lines().count() <= 1
            || !file_name.contains(".md")
        {
            continue;
        }
        let note: Note = file_content.parse().unwrap();
        let mut note: genanki_rs::Note = note.into();
        note = note.guid(file_name);
        deck.add_note(note);
    }
    deck.write_to_file(&format!("{}/{}.apkg", config.output_dir, config.deck_name))
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! note_conversion_tests {
        ($directory:literal { $($name:ident => $name_str:literal,)* }) => {
            $(
                #[test]
                fn $name() {
                    let data = include_str!(concat!($directory, "/", $name_str, "/input.md"));
                    let note: Note = data.parse().unwrap();
                    assert_eq!(note.title, include_str!(concat!($directory, "/", $name_str, "/title.html")));
                    assert_eq!(note.body, include_str!(concat!($directory, "/", $name_str, "/body.html")));
                    assert_eq!(note.tags, include_str!(concat!($directory, "/", $name_str, "/tags.txt")).split('\n').collect::<Vec<&str>>());
                }
            )*
        };
    }

    note_conversion_tests!(
        "test_data" {
            simple_note_test => "simple",
            complex_note_test => "complex",
        }
    );
}
