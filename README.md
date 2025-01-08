# GitHub Permalink Generator

A simple Rust application to generate GitHub permalinks for files in a git repository.
I use it with [Helix](https://helix-editor.com/) like this:

```
:sh gh-permalink %{filename:repo_rel} %{linenumber}
```

Using the [command expansion v2 PR](https://github.com/helix-editor/helix/pull/11164)

## Features

- Generates a GitHub permalink pointing to a specific commit for a given file in the repository.
- Automatically uses the current working directory as the context for the repository.
- Requires the [GitHub CLI (gh)](https://cli.github.com/) for repository information.

## Install

Make sure you have the the [GitHub CLI (gh)](https://cli.github.com/) installed.

Build `gh-permalink` with either `nix` or `cargo` and add the binary to your $PATH.

#### Nix

```bash
nix build github:cor/gh-permalink
```

#### Cargo

```bash
git clone https://github.com/cor/gh-permalink
cd gh-permalink
cargo build --release
```

## Usage

Run the command by providing the path of the file relative to the repository root:

```bash
gh-permalink <file-path>
```

### Example

```bash
gh-permalink src/main.rs
```

Output:

```text
https://github.com/<username>/<repository>/blob/<commit-hash>/src/main.rs
```
