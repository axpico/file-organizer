//! # File Sorter
//!
//! A command-line tool that categorizes files by their extensions.
//! It scans a directory and groups files into categories based on their file types.

mod file_extension_enum;

use std::collections::HashMap;
use std::env;
use std::fs::{self, DirEntry, ReadDir};
use std::path::PathBuf;
use std::process;

use file_extension_enum::FileExtension;

/// Main entry point for the application
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: file_sorter [directory_path]");
        process::exit(1);
    });

    println!("Scanning directory: {}", config.directory.display());

    match fs::read_dir(&config.directory) {
        Ok(paths) => {
            let sorted_files = divide_files(paths);
            display_results(&sorted_files);
        },
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            process::exit(1);
        }
    }
}

/// Configuration for the application
struct Config {
    directory: PathBuf,
}

impl Config {
    /// Creates a new Config from command-line arguments
    ///
    /// # Arguments
    ///
    /// * `args` - Command-line arguments
    ///
    /// # Returns
    ///
    /// * `Result<Config, &'static str>` - Config or error message
    fn new(args: &[String]) -> Result<Config, &'static str> {
        let directory = if args.len() > 1 {
            PathBuf::from(&args[1])
        } else {
            env::current_dir().map_err(|_| "Could not get current directory")?
        };

        if !directory.exists() {
            return Err("Directory does not exist");
        }

        if !directory.is_dir() {
            return Err("Path is not a directory");
        }

        Ok(Config { directory })
    }
}

/// Categorizes files by their extensions
///
/// # Arguments
///
/// * `paths` - Iterator over directory entries
///
/// # Returns
///
/// * `HashMap<String, Vec<DirEntry>>` - Map of category names to files
fn divide_files(paths: ReadDir) -> HashMap<String, Vec<DirEntry>> {
    let mut files_by_type = HashMap::<String, Vec<DirEntry>>::new();

    // Initialize categories
    let categories = [
        "Text Files", "Document Files", "Image Files", "Audio Files",
        "Video Files", "Archive Files", "Executable Files", "Code Files",
        "Web Files", "Database Files", "Virtual Machine & Disk Images",
        "Font Files", "3D Model Files", "Scientific Data",
        "Configuration and System Files", "Game Files",
        "Blockchain & Crypto", "Others"
    ];

    for category in categories {
        files_by_type.insert(category.to_string(), Vec::new());
    }

    // Process each file
    for entry_result in paths {
        if let Ok(entry) = entry_result {
            if entry.path().is_file() {
                if let Some(extension) = get_file_extension(&entry) {
                    let file_ext = FileExtension::from_str(&extension);
                    let category = categorize_extension(file_ext);

                    if let Some(files) = files_by_type.get_mut(category) {
                        files.push(entry);
                    }
                } else {
                    // Files with no extension
                    if let Some(files) = files_by_type.get_mut("Others") {
                        files.push(entry);
                    }
                }
            }
        }
    }

    files_by_type
}

/// Extracts file extension from a directory entry
///
/// # Arguments
///
/// * `entry` - Directory entry
///
/// # Returns
///
/// * `Option<String>` - File extension if present
fn get_file_extension(entry: &DirEntry) -> Option<String> {
    entry.path().extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
}

/// Maps a FileExtension to its category
///
/// # Arguments
///
/// * `ext` - File extension enum value
///
/// # Returns
///
/// * `&'static str` - Category name
fn categorize_extension(ext: FileExtension) -> &'static str {
    match ext {
        FileExtension::Txt | FileExtension::Md | FileExtension::Rtf |
        FileExtension::Log | FileExtension::Json | FileExtension::Xml |
        FileExtension::Yaml | FileExtension::Yml | FileExtension::Toml |
        FileExtension::Ini => "Text Files",

        FileExtension::Pdf | FileExtension::Doc | FileExtension::Docx |
        FileExtension::Xls | FileExtension::Xlsx | FileExtension::Ppt |
        FileExtension::Pptx | FileExtension::Odt | FileExtension::Ods |
        FileExtension::Odp => "Document Files",

        FileExtension::Jpg | FileExtension::Jpeg | FileExtension::Png |
        FileExtension::Gif | FileExtension::Bmp | FileExtension::Tiff |
        FileExtension::Webp | FileExtension::Svg | FileExtension::Ico |
        FileExtension::Psd | FileExtension::Ai | FileExtension::Eps |
        FileExtension::Raw => "Image Files",

        FileExtension::Mp3 | FileExtension::Wav | FileExtension::Ogg |
        FileExtension::Flac | FileExtension::Aac | FileExtension::M4a |
        FileExtension::Wma | FileExtension::Amr | FileExtension::Midi => "Audio Files",

        FileExtension::Mp4 | FileExtension::Mkv | FileExtension::Avi |
        FileExtension::Mov | FileExtension::Wmv | FileExtension::Flv |
        FileExtension::Webm | FileExtension::Mpeg | FileExtension::M4v => "Video Files",

        FileExtension::Zip | FileExtension::Rar | FileExtension::Tar |
        FileExtension::Gz | FileExtension::Bz2 | FileExtension::SevenZ |
        FileExtension::Dmg | FileExtension::Xz | FileExtension::Lz |
        FileExtension::Cab => "Archive Files",

        FileExtension::Exe | FileExtension::Sh | FileExtension::Bat |
        FileExtension::Cmd | FileExtension::AppImage | FileExtension::Jar |
        FileExtension::Bin | FileExtension::Run | FileExtension::Msi |
        FileExtension::Elf => "Executable Files",

        FileExtension::Rs | FileExtension::C | FileExtension::H |
        FileExtension::Cpp | FileExtension::Hpp | FileExtension::Java |
        FileExtension::Js | FileExtension::Ts | FileExtension::Py |
        FileExtension::Swift | FileExtension::Go | FileExtension::Rb |
        FileExtension::Pl | FileExtension::Lua | FileExtension::Kotlin |
        FileExtension::Dart | FileExtension::CSharp | FileExtension::Cs |
        FileExtension::Scala | FileExtension::Haskell | FileExtension::Hs |
        FileExtension::Lisp | FileExtension::R | FileExtension::Julia |
        FileExtension::Jl | FileExtension::Tcl | FileExtension::Perl |
        FileExtension::Pm => "Code Files",

        FileExtension::Html | FileExtension::Css | FileExtension::Jsx |
        FileExtension::Tsx | FileExtension::Php | FileExtension::Asp |
        FileExtension::Aspx => "Web Files",

        FileExtension::Sql | FileExtension::Sqlite | FileExtension::Mdb |
        FileExtension::Accdb | FileExtension::JsonDb => "Database Files",

        FileExtension::Vmdk | FileExtension::Vdi | FileExtension::Vhd |
        FileExtension::Vhdx | FileExtension::Qcow2 => "Virtual Machine & Disk Images",

        FileExtension::Ttf | FileExtension::Otf | FileExtension::Woff |
        FileExtension::Woff2 | FileExtension::Eot => "Font Files",

        FileExtension::Obj | FileExtension::Fbx | FileExtension::Stl |
        FileExtension::Blend | FileExtension::Glb | FileExtension::Gltf |
        FileExtension::Ply => "3D Model Files",

        FileExtension::Csv | FileExtension::Mat | FileExtension::Hdf5 |
        FileExtension::Nc | FileExtension::Dcm | FileExtension::Fits => "Scientific Data",

        FileExtension::Conf | FileExtension::Reg | FileExtension::Inf |
        FileExtension::Sys | FileExtension::Dll | FileExtension::Dat |
        FileExtension::Db => "Configuration and System Files",

        FileExtension::Save | FileExtension::Pak | FileExtension::Vpk |
        FileExtension::Wad | FileExtension::Rom | FileExtension::Iso |
        FileExtension::Nso => "Game Files",

        FileExtension::Wallet | FileExtension::Key | FileExtension::JsonKey |
        FileExtension::Pem | FileExtension::P12 => "Blockchain & Crypto",

        FileExtension::Unknown => "Others",
    }
}

/// Displays the categorized files
///
/// # Arguments
///
/// * `sorted_files` - Map of category names to files
fn display_results(sorted_files: &HashMap<String, Vec<DirEntry>>) {
    let mut total_files = 0;
    let mut non_empty_categories = 0;

    // Count files and categories first
    for files in sorted_files.values() {
        if !files.is_empty() {
            total_files += files.len();
            non_empty_categories += 1;
        }
    }

    println!("\nFound {} files in {} categories:\n", total_files, non_empty_categories);

    // Display files by category
    for (category, files) in sorted_files {
        if !files.is_empty() {
            println!("📁 {} ({} files)", category, files.len());
            for file in files {
                println!("  - {}", file.file_name().to_string_lossy());
            }
            println!();
        }
    }
}
