use clap::Parser;
use std::error::Error;
use std::fs::create_dir_all;
use std::fs::remove_file;
use std::fs::write;
use std::fs::File;
use std::io;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pyversion: String,

    #[arg(short, long)]
    savepath: Option<String>,

    #[arg(short, long)]
    requirements: Option<String>,

    #[arg(short, long)]
    compress: bool,
}

struct SemanticVersioning {
    major: String,
    minor: String,
    patch: String,
}

fn make_semantic_versioning(ver: &String) -> SemanticVersioning {
    let splited = ver.split(".");
    if splited.clone().collect::<Vec<&str>>().len() != 3 {
        panic!("version must be [major].[minor].[patch]")
    }
    let sv = SemanticVersioning {
        major: splited.clone().nth(0).unwrap().to_string(),
        minor: splited.clone().nth(1).unwrap().to_string(),
        patch: splited.clone().nth(2).unwrap().to_string(),
    };
    return sv;
}

fn distribute(
    pyversion: &String,
    savepath: &String,
    requirements: Option<String>,
    compress: bool,
) -> Result<(), io::Error> {
    let sv = make_semantic_versioning(pyversion);
    let zipfilepath = format!("python-{}-embed-amd64.zip", pyversion);
    download(
        format!(
            "https://www.python.org/ftp/python/{}/python-{}-embed-amd64.zip",
            pyversion, pyversion
        )
        .to_string(),
        zipfilepath.to_string(),
    );
    if Path::new(savepath).is_dir() {
        panic!("A folder may already exist at the savepath you specified.");
    }
    create_dir_all(savepath).unwrap();
    Command::new("tar")
        .arg("-xf")
        .arg(&zipfilepath)
        .arg("-C")
        .arg(savepath)
        .output()
        .expect("failed to execute process");
    remove_file(&zipfilepath).unwrap();

    let path = format!("{}/python{}{}._pth", savepath, sv.major, sv.minor);

    let body = format!(
        "python{}{}.zip\n.\n\n# Uncomment to run site.main() automatically\nimport site",
        sv.major, sv.minor
    );
    write(path, body).unwrap();

    download(
        "https://bootstrap.pypa.io/get-pip.py".to_string(),
        format!("{}/get-pip.py", savepath).to_string(),
    );

    Command::new("cmd")
        .arg("/C")
        .arg(format!("{}\\python.exe", savepath.replace("/", "\\")))
        .arg(format!("{}\\get-pip.py", savepath.replace("/", "\\")))
        .status()
        .expect("failed to execute process");

    match requirements {
        Some(path) => {
            Command::new("cmd")
                .arg("/C")
                .arg(format!("{}\\python.exe", savepath.replace("/", "\\")))
                .arg("-m")
                .arg("pip")
                .arg("install")
                .arg("-r")
                .arg(path.replace("/", "\\"))
                .output()
                .expect("failed to execute process");
        }
        None => (),
    }

    if compress == true {
        Command::new("tar.exe")
            .arg("-C")
            .arg(savepath.replace("/", "\\"))
            .arg("-caf")
            .arg(format!(
                "{}.zip",
                savepath.replace("/", "\\").split("\\").last().unwrap()
            ))
            .arg("*")
            .output()
            .expect("failed to execute process");
        std::fs::remove_dir_all(savepath.replace("/", "\\")).unwrap();
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let savepath = match args.savepath {
        Some(path) => path,
        None => format!("./python-{}-embed-amd64", args.pyversion),
    };
    distribute(&args.pyversion, &savepath, args.requirements, args.compress)?;
    Ok(())
}

pub fn download(url: String, savepath: String) {
    let response = reqwest::blocking::get(&url).expect("wrong url");
    let bytes = response.bytes().unwrap();
    let mut out = File::create(savepath).expect("path");
    io::copy(&mut bytes.as_ref(), &mut out).expect("copy failed");
}

#[cfg(test)]
mod tests {
    use crate::distribute;
    use std::fs::{remove_file, write};
    use std::{fs::remove_dir_all, process::Command};

    fn run_test(pyversion: &String) {
        let body = format!("numpy",);

        write(format!("{}_requirements.txt", pyversion), body).unwrap();
        distribute(
            &pyversion,
            &format!("test_{}/python-{}-embed-amd64", pyversion, pyversion).to_string(),
            Some(format!("{}_requirements.txt", pyversion)),
            false,
        )
        .unwrap();

        let status = Command::new(format!(
            "test_{}\\python-{}-embed-amd64\\python.exe",
            pyversion, pyversion
        ))
        .arg("-c")
        .arg("try:\n\timport numpy\nexcept:\n\traise")
        .status()
        .expect("failed to execute process");
        assert!(status.success());
        remove_file(format!("{}_requirements.txt", pyversion)).unwrap();
        remove_dir_all(format!("test_{}", pyversion)).unwrap();
    }

    #[test]
    fn test_3_11_0() {
        run_test(&"3.11.0".to_string());
    }

    #[test]
    fn test_3_10_8() {
        run_test(&"3.10.8".to_string());
    }

    #[test]
    fn test_3_9_13() {
        run_test(&"3.9.13".to_string());
    }
    #[test]
    fn test_3_8_10() {
        run_test(&"3.8.10".to_string());
    }
    #[test]
    fn test_3_7_9() {
        run_test(&"3.7.9".to_string());
    }
}
