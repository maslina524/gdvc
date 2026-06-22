use std::fs::{File, self};
use std::io::{copy, self};
use std::path::{Path, PathBuf};

use zip::ZipArchive;

use crate::ws::WsClient;
use crate::{files, level};



fn unzip<P: AsRef<Path>>(zip_path: P, extract_path: P) -> io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    fs::create_dir_all(&extract_path)?;

    for i in 0..archive.len() {
        let mut file_in_archive = archive.by_index(i)?;
        let outpath = extract_path.as_ref().join(file_in_archive.name());

        if file_in_archive.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&outpath)?;
            copy(&mut file_in_archive, &mut outfile)?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file_in_archive.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}

pub fn import(marker: Option<u32>, path: String) -> Result<(), Box<dyn std::error::Error>> {
    let marker = match marker {
        Some(m) => m,
        None => {
            let mut ws = WsClient::connect()?;
            let string = ws.get_level_string()?;
            let marker = level::get_marker(&string).ok_or("The level is not initialized".to_string())?;
            marker
        }
    };

    let path = PathBuf::from(path);

    let extract_path = files::get_level_path(marker);
    unzip(path, extract_path)?;

    Ok(())
}