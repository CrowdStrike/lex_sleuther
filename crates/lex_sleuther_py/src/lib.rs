use std::{path::PathBuf, str::FromStr};

use ::lex_sleuther::baked_model;

use pyo3::prelude::*;

/// The simplest possible API for getting file classifications.
/// Will probably throw an exception if the file paths in question does not exist.
#[pyfunction]
fn classify_files(filepaths: Vec<String>) -> PyResult<Vec<String>> {
    let paths = filepaths
        .into_iter()
        .map(|str| PathBuf::from_str(&str))
        .collect::<Result<Vec<_>, _>>()?;

    let baked_model = baked_model();
    let classifications = baked_model
        .classify_files(&paths)
        .iter()
        .map(|classification| classification.verdicts.get(0).unwrap().label.to_owned())
        .collect();

    Ok(classifications)
}

/// Use the default training set and classifications to classify the single file represented
/// by the provided byte array.
#[pyfunction]
fn classify_bytes(bytes: Vec<u8>) -> String {
    let baked_model = baked_model();
    baked_model.classify_bytes(&bytes).verdicts.get(0).unwrap().label.to_owned()
}

#[pymodule]
fn lex_sleuther(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(classify_files, m)?)?;
    m.add_function(wrap_pyfunction!(classify_bytes, m)?)?;
    Ok(())
}
