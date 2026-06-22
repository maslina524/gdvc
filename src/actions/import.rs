use std::fs::{File, self};
use std::io::{self, ErrorKind, copy};
use std::path::{Path, PathBuf};

use zip::ZipArchive;

use crate::ws::WsClient;
use crate::{files, level};

fn copy_dir_recursive(src_dir: &PathBuf, dst_dir: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(dst_dir)?;

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dest_path = dst_dir.join(file_name);

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

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

pub fn import(marker: Option<u32>, path: Option<String>, to_file: bool) -> Result<(), Box<dyn std::error::Error>> {
    let marker = match marker {
        Some(m) => m,
        None => {
            let mut ws = WsClient::connect()?;
            let string = ws.get_level_string()?;
            let marker = level::get_marker(&string).ok_or("The level is not initialized".to_string())?;
            marker
        }
    };

    if to_file {
        let file_path = files::get_gdvc_path().join("export_marker");
        let exported_marker_str = match fs::read_to_string(file_path) {
            Ok(m) => m,
            Err(e) => {
                return Err(match e.kind() {
                    ErrorKind::NotFound => "Previously there were no exported levels with the --to_file (-f) flag".into(),
                    _ => e.into()
                });
            }
        };
        let exported_marker = exported_marker_str.parse::<u32>()?;

        let src_dir = files::get_level_path(exported_marker);
        let dst_dir = files::get_level_path(marker);
        copy_dir_recursive(&src_dir, &dst_dir)?;
        return Ok(());
    }

    let path = PathBuf::from(path.unwrap());

    let extract_path = files::get_level_path(marker);
    unzip(path, extract_path)?;

    Ok(())
}