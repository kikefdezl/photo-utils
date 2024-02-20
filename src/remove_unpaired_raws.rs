use std::fs;
use std::path::PathBuf;

use dialoguer::Confirm;

const RAW_EXTENSIONS: [&str; 2] = ["RAW", "RAF"];
const JPEG_EXTENSIONS: [&str; 2] = ["JPG", "JPEG"];

pub fn remove_unpaired_raws(directory: &PathBuf) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(directory)?;

    let mut raws_to_remove = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(extension) = path.extension() {
            if RAW_EXTENSIONS.iter().any(|&ext| ext == extension) {
                let stem = path.file_stem().unwrap().to_string_lossy();

                let mut paired = false;
                for &jpeg_ext in &JPEG_EXTENSIONS {
                    let jpeg_path = directory.join(&stem as &str).with_extension(jpeg_ext);
                    if jpeg_path.exists() {
                        paired = true;
                        break;
                    }
                }
                if !paired {
                    raws_to_remove.push(path.clone());
                }
            }
        }
    }

    let num_raw_files = raws_to_remove.len();

    if num_raw_files == 0 {
        println!("No unpaired RAW files found.");
        return Ok(());
    }

    let prompt = format!(
        "The following action will remove <{}> RAW files, do you want to continue?",
        num_raw_files
    );
    if !Confirm::new()
        .with_prompt(prompt)
        .default(true)
        .interact()
        .unwrap()
    {
        println!("Operation cancelled.");
        return Ok(());
    }

    for raw in raws_to_remove {
        fs::remove_file(raw.clone())?;
        println!("Removed: {:?}", raw);
    }
    Ok(())
}
