<p align="center">
  <h1 align="center">RAT - Auto-Copy Tool</h1>
    <p align="center">An app for auto-managing your repo file !</p>
</p>

<p align="center">
    <img src="https://img.shields.io/badge/version-1.0-blue" alt="version">
    <img src="https://img.shields.io/github/v/release/hhertout/rat_tool.svg" />
    <a href="https://github.com/hhertout/rac_tool/actions">
      <img alt="Tests Passing" src="https://github.com/hhertout/rac_tool/actions/workflows/rust.yml/badge.svg" />
      <img alt="Tests Passing" src="https://github.com/hhertout/rac_tool/actions/workflows/release.yml/badge.svg" />
    </a>
</p>

## Presentation


RAT is a tool allowing you to automate file copying, automatic replacement of certain textual parts of a file.

All are the process is set from a yml configuration file.

You can configure it for processing globally, or more specifically by targeting a specific file name, or file path.

## Getting started

### Initialization
Install the executable file in your working directory.

To create the configuration file, run :

```bash
./<exec_name> init
```

It creates the config.yml in your current directory.

### Configuration

```yaml
# configuration file

# Base file path to process - Default is root directory
on: .

copy:
  # List all files to copy
  # - `base:dest`
  files:
    - hello.txt.example:hello.txt

replace:
  # Replace a string in all files, in all directory
  global:
    - hello mom:hello mom

  # Target files by name. By default, target all directory.
  # If you want to target a specifiq file, you must specify the correct path.
  # content:
  #     - `past:future`
  target:
    - file_name: hello.txt
      content:
        - hello world:hello mom
        - string to replace:string replaced
    - file_name: dir/example/hello.txt
      content:
        - hello mom:hello mom

# ignore directory during all the process
ignored_dir:
  - /.git/
  - /tests/
  - /node_modules/
  - /vendor/
  - /target/
```

### Running the tool

Simply run

```bash
./<exec_file> run
```

## Installation

- Install globally : 
```bash
cargo install --path .
```
- Install with executable file :

Build with ```cargo build --release``` and find the file in ```target/release/fs_rust``` or download the executable file from repository releases.

## Contribution

All ideas or help are welcome !

If you want to add more functionality or fix some bugs, you can : 
- Submit an issue of bugs or with your idea
- Submit a pull request, to push your branch on the repository and submit a PR.

Thanks for your contribution to this project.