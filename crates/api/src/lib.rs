use serde_json;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn save_json() -> std::io::Result<()> {
    let file = std::fs::File::create("val.json")?;
    serde_json::to_writer(file, &42).unwrap();
    Ok(())
}

pub fn save_input_number_as_json(val: i32) -> std::io::Result<()> {
    let file = std::fs::File::create("val.json")?;
    serde_json::to_writer(file, &val).unwrap();
    Ok(())
}

pub fn save_input_number_as_json_to_custom_path(val: i32, path: &str) -> std::io::Result<()> {
    let file = std::fs::File::create(path)?;
    serde_json::to_writer(file, &val).unwrap();
    Ok(())
}

pub fn index_to_json() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
