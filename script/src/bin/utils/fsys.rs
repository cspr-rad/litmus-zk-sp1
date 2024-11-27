use std::fs;

pub(crate) fn get_fixture_content(fname: String) -> String {
    let path = get_path_to_fixture(fname);

    fs::read_to_string(path).unwrap()
}

pub(crate) fn get_path_to_fixture(fname: String) -> String {
    format!("fixtures/{fname}")
}
