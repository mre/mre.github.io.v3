use anyhow::anyhow;
use duct::cmd;
use glob::glob;
use image::imageops::FilterType::Lanczos3;
use image::GenericImageView;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

// Width of images on blog.
const MAX_IMAGE_WIDTH: u32 = 650; // pixels

fn main() -> Result<(), Box<dyn Error>> {
    for entry in glob("content/**/raw/*")? {
        match entry {
            Ok(path) => handle(path)?,
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}

fn copy_original(path: &Path, out_file: &Path) -> Result<(), Box<dyn Error>> {
    if path
        .extension()
        .ok_or(anyhow!("Cannot get extension for {}", path.display()))?
        == "svg"
    {
        // Simply copy over SVG to target directory for now.
        // In the future we could use svgo to optimize here.
        println!("SVG");
        fs::copy(path, out_file)?;
        return Ok(());
    }

    let img = image::open(&path)?;
    // Adjust width
    if img.width() > MAX_IMAGE_WIDTH {
        let img = img.resize(MAX_IMAGE_WIDTH, 1000, Lanczos3);
        img.save(&out_file)?;
    } else {
        // Image is already in the correct format. Just copy over.
        fs::copy(path, &out_file)?;
    };
    Ok(())
}

fn handle(path: PathBuf) -> Result<(), Box<dyn Error>> {
    println!("{}", path.display());
    let filename = path
        .file_name()
        .ok_or(anyhow!("Unexpected file: {}", path.display()))?;
    let in_dir = path
        .parent()
        .ok_or(anyhow!("Unexpected dir: {}", path.display()))?;

    let out_dir = Path::new("static").join(in_dir);
    let out_file = out_dir.join(filename);

    println!("in_file = {:?}", filename);
    println!("in_dir = {:?}", in_dir);
    println!("out_dir = {:?}", out_dir);
    println!("out_file = {:?}", out_file);

    fs::create_dir_all(&out_dir)?;

    if !out_file.exists() {
        copy_original(&path, &out_file)?;
    }

    if path
        .extension()
        .ok_or(anyhow!("Cannot get extension for {}", path.display()))?
        == "svg"
    {
        // We're done for SVG here.
        return Ok(());
    }

    cmd!("cwebp", &out_file, "-o", &out_file.with_extension("webp")).run()?;
    cmd!(
        "cavif",
        "--quality=30",
        "--overwrite",
        "-o",
        &out_file.with_extension("avif"),
        &out_file
    )
    .run()?;

    //     create webp
    //     create avif
    //     cp image to out_dir
    //     cp image.webp to out_dir
    //     cp image.avif to out_dir
    Ok(())
}
