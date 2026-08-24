# Fugu 🐡

`fugu` is a small Rust CLI tool for analyzing file and directory sizes.

It recursively scans a directory, gets the total size of each directory, sorts entries by size, and prints the result as a tree.

## Usage
```
fugu <DIRECTORY_PATH>
```

where <DIRECTORY_PATH> is a path to the directory, that you want to analyze.

## Example
```
fugu ./root

./root 131.78KB
├── ./Goodbye 123.55KB
│   └── ./World 123.55KB
│       └── Goodbye World.txt 123.55KB
└── ./Hello 8.22KB
    ├── test.txt 8.21KB
    └── ./World 13B
        └── Hello World.txt 13B
```