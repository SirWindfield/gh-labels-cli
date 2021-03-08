use eyre::{Context, Result};
use hubcaps::labels::LabelOptions;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

#[derive(Debug, Deserialize, Serialize)]
pub struct Label {
    pub color: String,
    pub description: Option<String>,
    pub name: String,
}

impl From<Label> for LabelOptions {
    fn from(lbl: Label) -> Self {
        LabelOptions::new(lbl.name, lbl.color, lbl.description.unwrap_or_default())
    }
}

pub type Labels = Vec<Label>;

pub fn read_file(path: impl AsRef<Path>) -> Result<Labels> {
    let file = File::open(path.as_ref()).wrap_err_with(|| "Cannot find label definition file")?;
    serde_json::from_reader(file)
        .wrap_err_with(|| "Misformatted label definition file. Make sure the file is valid json!")
}
