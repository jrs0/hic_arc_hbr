//! The main Rust-language interface layer between rust_hic (the Rust crate)
//! and py_hic (the Python package).

use pyo3::prelude::*;
use rust_hic::{clinical_code::ClinicalCodeStore, clinical_code_tree::ClinicalCodeTree};
use std::collections::HashMap;


/// Get the clinical codes in a particular code group defined
/// in a codes file.
///
/// The result is a named list (intended as a dataframe) with the
/// columns:
/// * name: the name of the code in the group (e.g. A01.0)
/// * docs: the description of the code 
/// 
/// TODO: figure out a good way to handle errors.
/// 
/// @export
#[pyfunction]
fn rust_get_codes_in_group(codes_file_path: &str, group: &str) -> HashMap<String, Vec<String>> {
    let f = std::fs::File::open(codes_file_path).expect("Failed to open codes file");

    let code_tree = ClinicalCodeTree::from_reader(f);
    let mut code_store = ClinicalCodeStore::new();

    let clinical_code_refs = code_tree
        .codes_in_group(&String::from(group), &mut code_store)
        .expect("Should succeed, code is present");

    let mut name = Vec::new();
    let mut docs = Vec::new();
    for code_ref in clinical_code_refs {
        let clinical_code = code_store
            .clinical_code_from(&code_ref)
            .expect("Clinical code should be present");
        name.push(clinical_code.name().clone());
        docs.push(clinical_code.docs().clone());
    }

    let mut code_list = HashMap::new();
    code_list.insert(format!("name"), name);
    code_list.insert(format!("docs"), docs);
    code_list
}

/// Get the code groups defined in a codes file
/// 
/// Returns a character vector of group names defined in
/// the codes file. This can be used as the basis for fetching
/// all the code groups using rust_get_codes_in_group.
/// 
#[pyfunction]
fn rust_get_groups_in_codes_file(codes_file_path: &str) -> Vec<String> {
    let f = std::fs::File::open(codes_file_path).expect("Failed to open codes file");
    let code_tree = ClinicalCodeTree::from_reader(f);
    // get the code groups and return here
    code_tree.groups().iter().cloned().collect()
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name="_lib_name")]
fn my_lib_name(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_get_codes_in_group, m)?)?;
    m.add_function(wrap_pyfunction!(rust_get_groups_in_codes_file, m)?)?;
    Ok(())
}