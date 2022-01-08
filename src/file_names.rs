use std::path::PathBuf;

pub fn cop2molaxis(path: &String) -> PathBuf {
    let mut write_path = PathBuf::from(&path);

    let filename = String::from(write_path.file_name().unwrap().to_str().unwrap());
    write_path.pop();
    write_path.pop();
    write_path.push("2_molaxis");
    write_path.push(filename);
    write_path
}

pub fn cop2mcs(path: &String) -> PathBuf {
    let mut write_path = PathBuf::from(&path);

    let filename = String::from(write_path.file_name().unwrap().to_str().unwrap());
    write_path.pop();
    write_path.pop();
    write_path.push("1_decoded");
    write_path.push(filename);
    write_path.set_extension("lstdecoded");
    write_path
}