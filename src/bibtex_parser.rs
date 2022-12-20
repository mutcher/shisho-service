use std::ops::Add;

use crate::commands::{self, Book};

impl commands::Book {
    pub fn from_bibtex(input: &str) -> Result<Self, String> {
        if input.len() == 0 {
            return Err("String cannot be empty".to_owned());
        }

        let one_line_input = input.replace("\n", "");
        let mut trimmed_input = one_line_input.trim();
        let field_data_start = trimmed_input.find('{');

        if field_data_start.is_none() {
            return Err("Invalid BibTex format".to_owned());
        }

        trimmed_input = &trimmed_input[field_data_start.unwrap().add(1)..trimmed_input.len() - 1];
        let id_end = trimmed_input.find(',');

        if id_end.is_none() {
            return Err("Invalid BibTex format #2".to_owned());
        }

        let id = &trimmed_input[..id_end.unwrap()];
        println!("Found id: {}", id);

        trimmed_input = &trimmed_input[id_end.unwrap().add(1)..];
        let mut book = Book {
            id: id.to_string(),
            title: "".to_owned(),
            ..Default::default()
        };

        // Extracting fields data
        loop {
            let equation_index = trimmed_input.find("=");
            if equation_index.is_none() {
                break;
            }

            let field_name = &trimmed_input[..equation_index.clone().unwrap()].trim();
            let field_value;
            let value_symbol = &trimmed_input
                .chars()
                .nth(equation_index.clone().unwrap().add(1))
                .unwrap();
            let is_escaped_value = match value_symbol {
                '{' => true,
                _ => false,
            };
            if is_escaped_value {
                let end_value_symbol = &trimmed_input.find("}");
                if end_value_symbol.is_none() {
                    return Err("Invalid BibTex format #10".to_owned());
                }

                field_value = &trimmed_input
                    [equation_index.clone().unwrap().add(2)..end_value_symbol.unwrap()];
                if trimmed_input.len() > end_value_symbol.unwrap().add(2) {
                    trimmed_input = &trimmed_input[end_value_symbol.unwrap().add(2)..];
                } else {
                    break;
                }
            } else {
                let end_value_symbol = &trimmed_input.find(",");
                if end_value_symbol.is_none() {
                    return Err("Invalid BibTex format #100".to_owned());
                }

                field_value = &trimmed_input
                    [equation_index.clone().unwrap().add(2)..end_value_symbol.unwrap()];
                if trimmed_input.len() > end_value_symbol.unwrap().add(1) {
                    trimmed_input = &trimmed_input[end_value_symbol.unwrap().add(1)..];
                } else {
                    break;
                }
            };

            println!("Found pair \"{}\"=\"{}\"", field_name, field_value);

            match field_name {
                &"title" => book.title = field_value.to_owned(),
                &"url" => book.url = Some(field_value.to_owned()),
                &"author" => book.authors.push(field_value.to_owned()),
                &"year" => book.year = Some(u32::from_str_radix(field_value, 10).unwrap()),
                &"journal" => book.journal = Some(field_value.to_owned()),
                _ => println!("Unknown field \"{}\", skipping it", field_name),
            }
        }

        return Ok(book);
    }
}

#[test]
fn empty_string_should_fail() {
    assert_eq!(Book::from_bibtex("").is_err(), true);
}

#[test]
fn handles_invalid_input() {
    assert_eq!(
        Book::from_bibtex("@some"),
        Err("Invalid BibTex format".to_owned())
    )
}

#[test]
fn cite_citation_can_be_parsed() {
    assert_eq!(Book::from_bibtex(" @misc{kotlin help, title={Channels: Kotlin}, url={https://kotlinlang.org/docs/channels.html#building-channel-producers}, journal={Kotlin Help}}").unwrap() ,
     Book {
        id: "kotlin help".to_string(),
        title: "Channels: Kotlin".to_owned(),
        authors: vec![],
        url: Some("https://kotlinlang.org/docs/channels.html#building-channel-producers".to_owned()),
        year: None,
        journal: None
    });
}

#[test]
fn cite_multiline_can_be_parsed() {
    let string_value = r#"@article{levitt2011towards,
        title={Towards project management 2.0},
        author={Levitt, Raymond E},
        journal={Engineering project organization journal},
        volume={1},
        number={3},
        pages={197--210},
        year={2011},
        publisher={Taylor \& Francis}
      }"#;

    assert_eq!(
        Book::from_bibtex(string_value),
        Ok(Book {
            id: "levitt2011towards".to_owned(),
            title: "Towards project management 2.0".to_owned(),
            authors: vec!["Levitt, Raymond E".to_string()],
            year: Some(2011),
            journal: Some("Engineering project organization journal".to_owned()),
            ..Default::default()
        })
    );
}
