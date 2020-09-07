use anyhow::anyhow;
use duct::cmd;
use glob::glob;
use image::imageops::FilterType::Lanczos3;
use image::{GenericImageView, ImageFormat};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

// Width of images on blog.
const MAX_IMAGE_WIDTH: u32 = 650; // pixels
const INPUT_PATH: &'static str = "content/**/raw/*";

fn main() -> Result<(), Box<dyn Error>> {
    for entry in glob(INPUT_PATH)? {
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
    let mut img = image::open(&path)?;
    // Adjust width
    if img.width() > MAX_IMAGE_WIDTH {
        img = img.resize(MAX_IMAGE_WIDTH, 1000, Lanczos3);
    }
    img.save_with_format(out_file.with_extension("jpg"), ImageFormat::Jpeg)?;
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

    // This is not so beautiful...
    let out_dir = Path::new("static").join(
        in_dir
            .strip_prefix("content/")?
            .parent()
            .ok_or(anyhow!("Cannot get parent for {}", in_dir.display()))?,
    );
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
    Ok(())
}