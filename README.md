# File Organizer

A command-line tool that categorizes files by their extensions. It scans a directory and groups files into categories based on their file types.

## Features

- Categorizes files into 18 different groups based on file extensions
- Supports over 100 common file extensions
- Provides a clean, organized output of file categories
- Works with any directory accessible from the command line
- Simple, user-friendly interface

## Categories

Files are organized into the following categories:

- Text Files
- Document Files
- Image Files
- Audio Files
- Video Files
- Archive Files
- Executable Files
- Code Files
- Web Files
- Database Files
- Virtual Machine & Disk Images
- Font Files
- 3D Model Files
- Scientific Data
- Configuration and System Files
- Game Files
- Blockchain & Crypto
- Others (for unrecognized extensions)

## Installation

### From Source

1. Make sure you have Rust and Cargo installed. If not, install from [rustup.rs](https://rustup.rs/).
2. Clone this repository:
   ```
   git clone https://github.com/yourusername/file-organizer.git
   cd file-organizer
   ```
3. Build the project:
   ```
   cargo build --release
   ```
4. The executable will be available at `target/release/file-organizer`

## Usage

```
file-organizer [directory_path]
```

If no directory is specified, the current directory will be scanned.

### Examples

Scan the current directory:
```
file-organizer
```

Scan a specific directory:
```
file-organizer /path/to/directory
```

## Output Example

```
Scanning directory: /home/user/documents

Found 42 files in 5 categories:

📁 Document Files (15 files)
  - report.pdf
  - presentation.pptx
  - spreadsheet.xlsx
  - document.docx

📁 Image Files (12 files)
  - photo.jpg
  - logo.png
  - icon.svg
  - banner.gif

📁 Code Files (8 files)
  - main.rs
  - app.js
  - styles.css
  - index.html

📁 Text Files (5 files)
  - notes.txt
  - readme.md
  - config.json
  - data.csv

📁 Others (2 files)
  - unknown_file
  - .hidden_file
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
