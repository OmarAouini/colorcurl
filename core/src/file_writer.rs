use std::fs;

/// Write to a file given a file path
pub fn write(data :&String, file_path :&String) -> Result<String, String> {
    match fs::write(format!("{}/ccurl_data.txt",file_path), data) {
        Ok(()) => Ok(format!("wrote result on file: {:?}", file_path)),
        Err(e) => Err(format!("unable to write file! cause:\n{:?}",e)),
    }
}