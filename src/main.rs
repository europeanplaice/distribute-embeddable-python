use clap::Parser;
use std::error::Error;
use std::path::Path;
use tokio::fs::create_dir;
use tokio::fs::write;
use tokio::fs::File;
use tokio::io;
use tokio::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pyversion: String,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let sv = make_semantic_versioning(&args.pyversion);
    download(
        format!(
            "https://www.python.org/ftp/python/{}/python-{}-embed-amd64.zip",
            args.pyversion, args.pyversion
        )
        .to_string(),
        format!("python-{}-embed-amd64.zip", args.pyversion).to_string(),
    )
    .await?;
    if Path::new(&format!("./python-{}-embed-amd64", args.pyversion)).is_dir() == false {
        create_dir(format!("./python-{}-embed-amd64", args.pyversion)).await?;
    }

    let mut child = Command::new("tar")
        .arg("-xf")
        .arg(format!("python-{}-embed-amd64.zip", args.pyversion))
        .arg("-C")
        .arg(format!("./python-{}-embed-amd64", args.pyversion))
        .spawn()
        .expect("failed to execute process");
    child.wait().await?;

    let path = format!(
        "./python-{}-embed-amd64/python{}{}._pth",
        args.pyversion, sv.major, sv.minor
    );

    let body = format!(
        "python{}{}.zip\n.\n\n# Uncomment to run site.main() automatically\nimport site",
        sv.major, sv.minor
    );
    write(path, body).await?;

    download(
        "https://bootstrap.pypa.io/get-pip.py".to_string(),
        format!("python-{}-embed-amd64/get-pip.py", args.pyversion).to_string(),
    )
    .await?;

    let mut get_pip = Command::new("cmd")
        .arg("/C")
        .arg(format!("python-{}-embed-amd64\\python.exe", args.pyversion))
        .arg(format!("python-{}-embed-amd64\\get-pip.py", args.pyversion))
        .spawn()
        .expect("failed to execute process");
    get_pip.wait().await?;

    Ok(())
}

async fn download(url: String, savepath: String) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;
    let mut out = File::create(savepath).await?;
    io::copy(&mut bytes.as_ref(), &mut out).await?;
    Ok(())
}
