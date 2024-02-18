# About

This is a simple tool that can be used to connect a new repository to a GitHub repository.

Basically it does these steps that are suggested for you each time you create a repository on GitHub.

```sh
git add .
git commit -m "init"
git branch -M main
git remote add origin <origin>
git push -u origin main
```

## Installation

```sh
cargo install --git https://github.com/ollivarila/setup-gh.git
```

## Usage

See

```sh
setup-gh --help
```
