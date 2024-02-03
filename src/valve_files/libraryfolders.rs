use std::{borrow::Cow, collections::HashMap};

use keyvalues_parser::Vdf;
use keyvalues_serde::from_vdf;
use serde::Deserialize;

use super::error::ValveFileError;

#[derive(Deserialize, Debug, PartialEq)]
pub struct LibraryFolders {
    pub libraries: Vec<Library>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Library {
    pub path: String,
    pub label: String,
    #[serde(rename = "contentid")]
    pub content_id: String,
    #[serde(rename = "totalsize")]
    pub total_size: u128,
    pub update_clean_bytes_tally: String,
    pub time_last_update_corruption: String,
    pub apps: HashMap<u64, u128>,
}

pub fn from_str(vdf_text: &str) -> Result<LibraryFolders, ValveFileError> {
    let mut vdf =
        Vdf::parse(vdf_text).map_err(|err| ValveFileError::KeyValuesParser(err.to_string()))?;
    let obj = vdf.value.get_mut_obj().ok_or(ValveFileError::Unknown)?;

    let mut index = 0;
    while let Some(mut library) = obj.remove(index.to_string().as_str()) {
        obj.entry(Cow::from("libraries"))
            .or_insert(Vec::new())
            .push(library.pop().ok_or(ValveFileError::Unknown)?);

        index += 1;
    }

    from_vdf(vdf).map_err(|err| ValveFileError::KeyValuesSerde(err.to_string()))
}

#[test]
fn test_de() {
    let data = include_str!("../../test-files/libraryfolders.vdf");

    let vdf = from_str(data);
    assert!(vdf.as_ref().is_ok());
    assert!(vdf
        .as_ref()
        .unwrap()
        .libraries
        .iter()
        .all(|x| !x.path.is_empty()));
    assert!(vdf
        .as_ref()
        .unwrap()
        .libraries
        .iter()
        .all(|x| !x.apps.is_empty()));
}
