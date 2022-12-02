# Distribute Embeddable Python

**Embeddable Python + pip + libraries**

The embeddable package is intended for acting as part of another application and 
doesn't even include `pip` at the time it is downloaded from python.org. This library downloads the embeddable python and configures it so that you can use with strong third party libraries out-of-the-box. If you prefer, it can compress the embeddable package folder to a zipfile and you can easily pass it to others.


## What it does inside
* Downloads the embeddable python from python.org and unzip
* Enables `import site`
* Downloads `get-pip.py` and install `pip`
* (Optional) installs other packages in `requirements.txt`
* (Optional) compresses the python into single zip file.

## Usage
1. Install Rust from https://www.rust-lang.org/
2. Clone this repository and change your current directory to this repo
3. Build Rust and make an executable
   * run `cargo build --release`
   * the executable will be created at `./target/release/distribute_embeddable_python.exe`
4. Run the executable

```
distribute_embeddable_python.exe [OPTIONS] --pyversion <PYVERSION>

Options:
  -p, --pyversion: <PYVERSION> python version e.g. 3.11.0
  -s, --savepath: <SAVEPATH> (optional) where to save the configured python
  -r, --requirements: <REQUIREMENTS> (optional) requirements.txt path to install libraries from
  -c, --compress: (optional) compresses the python into single zip file.
```
without `--savepath`, the folder is created in your current directory as `python-{pyversion}-embed-amd64`.

### example
`distribute_embeddable_python.exe --pyversion 3.10.8 --compress`


## Test

|version|status
|--|--|
|3.11.0|☑ Passed|
|3.10.8|☑ Passed|
|3.9.13|☑ Passed|
|3.8.10|☑ Passed|
|3.7.9|☑ Passed|
