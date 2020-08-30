use anyhow::anyhow;
use glob::glob;
use std::{
    error::Error,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    for entry in glob("content/**/raw/*")? {
        match entry {
            Ok(path) => handle(path)?,
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}

// fn extension(filename: &str) -> Option<&str> {
//     Path::new(filename)
//         .extension()
//         .and_then(OsStr::to_str)
// }

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

    fs::create_dir_all(&out_dir)?;

    if path
        .extension()
        .ok_or(anyhow!("Cannot get extension for {}", path.display()))?
        == "svg"
    {
        // Simply copy over SVG to target directory for now.
        // In the future we could use svgo to optimize here.
        println!("SVG");
        fs::copy(path, out_file)?;
    }

    //     adjust width
    //     create webp
    //     create avif
    //     cp image to out_dir
    //     cp image.webp to out_dir
    //     cp image.avif to out_dir
    Ok(())
}
