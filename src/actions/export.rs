use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

use crate::ws::WsClient;
use crate::{files, level};

use walkdir::WalkDir;
use zip::write::{ZipWriter, SimpleFileOptions};
use zip::CompressionMethod;

fn zip_directory(input_dir: &PathBuf, output_zip: &PathBuf) -> zip::result::ZipResult<()> {
    let file = File::create(output_zip)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated);

    for entry in WalkDir::new(input_dir) {
        let entry = entry.map_err(|e| zip::result::ZipError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        let path = entry.path();
        let name = path.strip_prefix(input_dir).unwrap();
        if name.as_os_str().is_empty() { continue; }

        let name_str = name.to_string_lossy().into_owned();
        if path.is_dir() {
            zip.add_directory(format!("{}/", name_str), options)?;
        } else if path.is_file() {
            zip.start_file(name_str, options)?;
            let mut f = File::open(path)?;
            copy(&mut f, &mut zip)?;
        }
    }
    zip.finish()?;
    Ok(())
}

pub fn export(marker: Option<u32>, path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let marker = match marker {
        Some(m) => m,
        None => {
            let mut ws = WsClient::connect()?;
            let string = ws.get_level_string()?;
            let marker = level::get_marker(&string).ok_or("The level is not initialized".to_string())?;
            marker
        }
    };

    let path = match path {
        Some(p) => PathBuf::from(p),
        None => std::env::current_dir()?
    };

    let input_dir = files::get_level_path(marker);
    let output_zip = path.join(format!("{marker}_exported.zip"));
    
    zip_directory(&input_dir, &output_zip)?;

    Ok(())
}