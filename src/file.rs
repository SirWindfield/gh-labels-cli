use eyre::{Context, Result};
use hubcaps::labels::LabelOptions;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonLabel {
    pub color: String,
    #[serde(default)]
    pub description: String,
    pub name: String,
}

impl JsonLabel {
    pub fn from(color: String, description: String, name: String) -> Self {
        Self {
            color,
            description,
            name,
        }
    }
}

impl From<JsonLabel> for LabelOptions {
    fn from(lbl: JsonLabel) -> Self {
        LabelOptions::new(lbl.name, lbl.color, lbl.description)
    }
}

impl PartialEq<hubcaps::labels::Label> for JsonLabel {
    fn eq(&self, other: &hubcaps::labels::Label) -> bool {
        other.color == self.color
            && other.description.as_deref() == Some(&self.description)
            && other.name == self.name
    }
}

pub type JsonLabels = Vec<JsonLabel>;

pub fn read_file(path: impl AsRef<Path>) -> Result<JsonLabels> {
    let file = File::open(path.as_ref()).wrap_err_with(|| "Cannot find label definition file")?;
    serde_json::from_reader(file)
        .wrap_err_with(|| "Misformatted label definition file. Make sure the file is valid json!")
}
