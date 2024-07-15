use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

const COMMIT: &str = "108a8f8";

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let archive = download_archive(&out, &target, None).unwrap();
    extract(&out, &archive);
    if &target == "x86_64-unknown-linux-gnu" {
        // libEGL.so.1 is read first in glutin
        std::fs::copy(out.join("libEGL.so"), out.join("libEGL.so.1")).unwrap();
    }
    println!("cargo:rustc-link-search={}", out.display());
    //println!("cargo:rustc-link-lib=libEGL");
}

fn download_archive(
    out: &Path,
    target: &str,
    base: Option<&str>,
) -> Result<PathBuf, std::io::Error> {
    let base = base.unwrap_or("https://github.com/sagudev/prebuild-angle/releases/download");
    let archive_path = out.join("angle.gz");
    let archive = format!("{base}/angle-{COMMIT}/ANGLE-{COMMIT}-{target}.zip");
    if !archive_path.exists()
        && !Command::new("curl")
            .arg("-L")
            .arg("-f")
            .arg("-s")
            .arg("-o")
            .arg(&archive_path)
            .arg(archive)
            .status()?
            .success()
    {
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    }

    Ok(archive_path)
}

fn extract(out: &Path, p: &Path) {
    let file = File::open(p).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };
        let outpath = out.join(outpath);

        if file.is_dir() {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}
