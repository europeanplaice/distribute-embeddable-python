# Distribute Embeddable Python

**Embeddable Python + pip + libraries All in One**

The embeddable package is intended for acting as part of another application and 
doesn't even include `pip` at the time it is downloaded from python.org. This library downloads the embeddable python and configures it so that you can use with strong third party libraries out-of-the-box. If you prefer, it can compress the embeddable package folder to a zipfile and you can easily pass it to others.

## Demo

https://user-images.githubusercontent.com/38364983/205297243-6a1b698b-4b8b-40c8-b4c9-fa1b69d7aeea.mp4


## What it does inside
* Downloads the embeddable python from python.org and unzip
* Enables `import site`
* Downloads `get-pip.py` and install `pip`
* (Optional) installs other packages in `requirements.txt`
* (Optional) compresses the python into a single zip file.

## Download embeddable python with pip installed
The python executables with pip installed available (Only amd64).
|version|link
|--|--|
|3.11.0|https://github.com/europeanplaice/distribute-embeddable-python/releases/download/v3.11.0/python-3.11.0-embed-amd64.zip
|3.10.8|https://github.com/europeanplaice/distribute-embeddable-python/releases/download/v3.10.8/python-3.10.8-embed-amd64.zip
|3.9.13|https://github.com/europeanplaice/distribute-embeddable-python/releases/download/v3.9.13/python-3.9.13-embed-amd64.zip
|3.8.10|https://github.com/europeanplaice/distribute-embeddable-python/releases/download/v3.8.10/python-3.8.10-embed-amd64.zip
|3.7.9|https://github.com/europeanplaice/distribute-embeddable-python/releases/download/v3.7.9/python-3.7.9-embed-amd64.zip

## Make your own python with pip and libraries
1. Install Rust from https://www.rust-lang.org/
2. Clone this repository and change your current directory to this repo
3. Build Rust and make an executable
   * run `cargo build --release`
   * the executable will be created at `./target/release/distribute_embeddable_python.exe`
4. Run the executable

```
Usage: distribute_embeddable_python.exe [OPTIONS] --pyversion <PYVERSION>

Options:
  -p, --pyversion <PYVERSION>        Python version e.g. 3.11.0
      --cpu <CPU>                    Cpu type to install. it must be 'win32' or 'amd64' or 'arm64' (arm64 only if pyversion >= 3.11). Defaults to 'amd64'
  -s, --savepath <SAVEPATH>          where to save the python. Defaults to './python-{pyversion}-embed-{cpu}'
  -r, --requirements <REQUIREMENTS>  requirements.txt path to install libraries from. if not specified, no libraries will be installed
  -i, --install <INSTALL>            libraries to install. If you install multiple libraries, the command must be surrounded by "". This is ignored if requirements is set.
  -c, --compress                     if true it compresses the python into a single zip file.
  -h, --help                         Print help information
  -V, --version                      Print version information
```

### example
`distribute_embeddable_python.exe --pyversion 3.10.8 --compress --install "pandas numpy tqdm" `
`distribute_embeddable_python.exe --pyversion 3.10.8 --requirements ./requirements.txt `

## How to install libraries manually
Because PATH of the embeddable python or pip is not added, you can't call pip as you usually do like `pip install numpy`. 

Instead, you can call pip from the path of python such as `python-3.11.0-embed-amd64\python.exe -m pip install numpy`

## Test

|version|status
|--|--|
|3.11.0|☑ Passed|
|3.10.8|☑ Passed|
|3.9.13|☑ Passed|
|3.8.10|☑ Passed|
|3.7.9|☑ Passed|
