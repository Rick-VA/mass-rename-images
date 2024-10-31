use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let folder_path = "./folder-name"; // Change this to your folder path

    // Collect all image files in the folder
    let mut images: Vec<_> = fs::read_dir(folder_path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && is_image(&path) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Sort images alphabetically
    images.sort();

    // Rename each image file to a sequence number
    for (index, image_path) in images.iter().enumerate() {
        let new_name = format!("{}.{}", index + 1, image_path.extension().unwrap().to_string_lossy());
        let new_path = Path::new(folder_path).join(new_name);
        fs::rename(image_path, new_path)?;
    }

    println!("Renamed {} files in {}", images.len(), folder_path);
    Ok(())
}

// Helper function to check if a file is an image
fn is_image(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("png") | Some("jpg") | Some("jpeg") => true,
        _ => false,
    }
}
