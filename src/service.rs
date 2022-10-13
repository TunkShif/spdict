use scraper::{Html, Selector};
use serde_json::Value;

use crate::models::*;

const QUERY_URL: &str = "https://www.spanishdict.com/translate/";
const DATA_TAG: &str = "window.SD_COMPONENT_DATA = ";

#[derive(Debug)]
pub enum Error {
    NetworkError,
    NotFoundError(String),
}

fn request(name: &str) -> Result<Option<Value>, Error> {
    let body = ureq::get(format!("{}{}", QUERY_URL, name).as_str())
        .call()
        .map_err(|_| Error::NetworkError)?
        .into_string()
        .unwrap();

    let data = Html::parse_document(&body)
        .select(&Selector::parse("script").unwrap())
        .find(|script| script.inner_html().trim().starts_with(DATA_TAG))
        .map(|e| e.inner_html())
        .as_ref()
        .map(|str| str.trim())
        .and_then(|str| str.strip_prefix(DATA_TAG))
        .and_then(|str| str.strip_suffix(';'))
        .and_then(|str| serde_json::from_str(str).unwrap_or(None));

    Ok(data)
}

pub fn query(name: &str) -> Result<Word, Error> {
    let raw_data = request(name)?;
    let data = raw_data
        .as_ref()
        .map(|json| &json["sdDictionaryResultsProps"]["entry"]["neodict"])
        .and_then(|json| if json.is_null() { None } else { Some(json) })
        .ok_or_else(|| Error::NotFoundError(name.to_string()))?;

    let word = data
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|node| node["subheadword"].as_str())
        .unwrap()
        .to_owned();

    let mut definitions: Vec<Definition> = vec![];
    for entry in data.as_array().unwrap() {
        for pos_item in entry["posGroups"].as_array().unwrap() {
            for sense_item in pos_item["senses"].as_array().unwrap() {
                for translation_item in sense_item["translations"].as_array().unwrap() {
                    let context = sense_item["contextEn"].as_str().unwrap().to_owned();

                    let pos = sense_item["partOfSpeech"]["nameEn"]
                        .as_str()
                        .unwrap()
                        .to_owned();

                    let translation = translation_item["translation"].as_str().unwrap().to_owned();

                    let sense = match (
                        context.as_str().trim().is_empty(),
                        translation.as_str().trim().is_empty(),
                    ) {
                        (true, _) => translation,
                        (false, true) => format!("({}) no direct translation", context),
                        (false, false) => format!("({}) {}", context, translation),
                    };

                    let examples = translation_item["examples"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|t| Example {
                            es_text: t["textEs"].as_str().unwrap().to_owned(),
                            en_text: t["textEn"].as_str().unwrap().to_owned(),
                        })
                        .collect::<Vec<Example>>();

                    definitions.push(Definition {
                        pos,
                        sense,
                        examples,
                    })
                }
            }
        }
    }

    Ok(Word {
        name: word,
        definitions,
    })
}

// pub fn conjugate(name: &str) {
//     unimplemented!()
// }
