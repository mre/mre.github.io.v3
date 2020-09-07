use anyhow::{anyhow, Result};
use duct::cmd;
use glob::glob;
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

// Width of images on blog.
const MAX_IMAGE_WIDTH: u32 = 650; // pixels
const INPUT_PATH: &'static str = "content/**/raw/*";

fn main() -> Result<()> {
    let entries: Vec<PathBuf> = glob(INPUT_PATH)?.filter_map(Result::ok).collect();
    entries
        .into_par_iter()
        .map(|entry| handle(entry))
        .collect::<Vec<_>>();
    Ok(())
}

fn copy_original(path: &Path, out_file: &Path) -> Result<()> {
    let ext = path
        .extension()
        .ok_or(anyhow!("Cannot get extension for {}", path.display()))?;

    if ext == "svg" || ext == "gif" {
        // Simply copy over SVG to target directory for now.
        // In the future we could use svgo to optimize here.
        println!("SVG");
        fs::copy(path, out_file)?;
        return Ok(());
    }

    cmd!(
        "convert",
        &path,
        "-adaptive-resize",
        MAX_IMAGE_WIDTH.to_string() + ">",
        out_file
    )
    .run()?;

    if ext == "png" {
        let cmd = cmd!(
            "cjpeg",
            "-quality",
            "85",
            "-optimize",
            "-outfile",
            out_file.with_extension("jpg"),
            out_file,
        );
        dbg!(&cmd);
        cmd.run()?;
    }

    Ok(())
}

fn handle(path: PathBuf) -> Result<()> {
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

    dbg!(&filename);
    dbg!(&in_dir);
    dbg!(&out_dir);

    if filename == ".DS_Store" {
        return Ok(());
    }

    let orig_extension = path
        .extension()
        .ok_or(anyhow!("Cannot get extension for {}", path.display()))?;

    if orig_extension == "afdesign" {
        return Ok(());
    }

    fs::create_dir_all(&out_dir)?;

    dbg!(&out_file);

    if !out_file.exists() {
        copy_original(&path, &out_file)?;
    }

    if orig_extension == "svg" || orig_extension == "gif" {
        // We're done here.
        return Ok(());
    }

    let webp_file = out_file.with_extension("webp");
    if !webp_file.exists() {
        cmd!("cwebp", &out_file, "-o", webp_file).run()?;
    }

    let avif_file = out_file.with_extension("avif");
    if !avif_file.exists() {
        cmd!(
            "cavif",
            "--quality=80",
            "--speed=2",
            "--overwrite",
            "-o",
            avif_file,
            &out_file
        )
        .run()?;
    }
    Ok(())
}
