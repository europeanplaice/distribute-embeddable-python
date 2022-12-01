use clap::Parser;
use std::error::Error;
use std::fs::create_dir_all;
use std::fs::remove_file;
use std::fs::write;
use std::fs::File;
use std::io;
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
}

struct SemanticVersioning {
    major: String,
    minor: String,
    patch: String,
}

fn make_semantic_versioning(ver: &String) -> SemanticVersioning {
    let splited = ver.split(".");
    let sv = SemanticVersioning {
        major: splited.clone().nth(0).unwrap().to_string(),
        minor: splited.clone().nth(1).unwrap().to_string(),
        patch: splited.clone().nth(2).unwrap().to_string(),
    };
    return sv;
}

fn distribute(pyversion: &String, savepath: &String, requirements: Option<String>) {
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
    match create_dir_all(savepath) {
        Ok(_) => (),
        Err(_) => panic!("A folder already exists at the savepath you specify."),
    };
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
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let savepath = match args.savepath {
        Some(path) => path,
        None => format!("./python-{}-embed-amd64", args.pyversion),
    };
    distribute(&args.pyversion, &savepath, args.requirements);
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
        let body = format!("requests",);

        write(format!("{}_requirements.txt", pyversion), body).unwrap();
        distribute(
            &pyversion,
            &format!("test_{}/python-{}-embed-amd64", pyversion, pyversion).to_string(),
            Some(format!("{}_requirements.txt", pyversion)),
        );

        let status = Command::new(format!(
            "test_{}\\python-{}-embed-amd64\\python.exe",
            pyversion, pyversion
        ))
        .arg("-c")
        .arg("try:\n\timport requests\nexcept:\n\traise")
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
    fn test_3_10_7() {
        run_test(&"3.10.7".to_string());
    }
}
