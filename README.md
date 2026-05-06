![gdvc banner](logo.png)

# Geometry Dash Version Control

git for Geometry Dash levels

A CLI tool for managing level versions with intuitive commands you already know.

## Installation

### From source

```bash
git clone https://github.com/maslina524/gdvc.git
cd gdvc
cargo build --release
```

## Quick Start

All actions are performed only when you are in the editor and you have the [WSLiveEditor](https://geode-sdk.org/mods/iandyhd3.wsliveeditor) mod installed.

Initialize your level for Gdvc:

```bash
gdvc init
```

Make the first commit:

```bash
gdvc commit -m "Initial commit"
```

Roll back progress to the previous commit:

```bash
gdvc rollback HEAD~1
```

## Todo List

- [ ] Restoring the gdvc signature in the level
    - [ ] by timestamp (manual recovery)
    - [ ] by .gmd file
- [ ] Auto detection of WS port
    - [ ] Windows
    - [ ] MacOS
- [ ] Branches
- [x] Log: commit history
- [ ] Python file for quick install