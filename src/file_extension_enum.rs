
#[derive(Debug, Clone, Copy)]
pub enum FileExtension {
    // Text Files
    Txt, Md, Rtf, Log, Json, Xml, Yaml, Yml, Toml, Ini,

    // Document Files
    Pdf, Doc, Docx, Xls, Xlsx, Ppt, Pptx, Odt, Ods, Odp,

    // Image Files
    Jpg, Jpeg, Png, Gif, Bmp, Tiff, Webp, Svg, Ico, Psd, Ai, Eps, Raw,

    // Audio Files
    Mp3, Wav, Ogg, Flac, Aac, M4a, Wma, Amr, Midi,

    // Video Files
    Mp4, Mkv, Avi, Mov, Wmv, Flv, Webm, Mpeg, M4v,

    // Archive Files
    Zip, Rar, Tar, Gz, Bz2, SevenZ, Dmg, Xz, Lz, Cab,

    // Executable Files
    Exe, Sh, Bat, Cmd, AppImage, Jar, Bin, Run, Msi, Elf,

    // Code Files
    Rs, C, H, Cpp, Hpp, Java, Js, Ts, Py, Swift, Go, Rb, Pl, Lua, Kotlin, Dart,
    CSharp, Cs, Scala, Haskell, Hs, Lisp, R, Julia, Jl, Tcl, Perl, Pm,

    // Web Files
    Html, Css, Jsx, Tsx, Php, Asp, Aspx,

    // Database Files
    Sql, Sqlite, Mdb, Accdb, JsonDb,

    // Virtual Machine & Disk Images
    Vmdk, Vdi, Vhd, Vhdx, Qcow2,

    // Font Files
    Ttf, Otf, Woff, Woff2, Eot,

    // 3D Model Files
    Obj, Fbx, Stl, Blend, Glb, Gltf, Ply,

    // Scientific Data
    Csv, Mat, Hdf5, Nc, Dcm, Fits,

    // Configuration and System Files
    Conf, Reg, Inf, Sys, Dll, Dat, Db,

    // Game Files
    Save, Pak, Vpk, Wad, Rom, Iso, Nso,

    // Blockchain & Crypto
    Wallet, Key, JsonKey, Pem, P12,

    // Others
    Unknown,
}

impl FileExtension {
    pub fn from_str(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "txt" => FileExtension::Txt, "md" => FileExtension::Md, "rtf" => FileExtension::Rtf, "log" => FileExtension::Log,
            "csv" => FileExtension::Csv, "json" => FileExtension::Json, "xml" => FileExtension::Xml, "yaml" | "yml" => FileExtension::Yaml,
            "toml" => FileExtension::Toml, "ini" => FileExtension::Ini, "conf" => FileExtension::Conf,

            "pdf" => FileExtension::Pdf, "doc" => FileExtension::Doc, "docx" => FileExtension::Docx,
            "xls" => FileExtension::Xls, "xlsx" => FileExtension::Xlsx, "ppt" => FileExtension::Ppt, "pptx" => FileExtension::Pptx,
            "odt" => FileExtension::Odt, "ods" => FileExtension::Ods, "odp" => FileExtension::Odp,

            "jpg" | "jpeg" => FileExtension::Jpg, "png" => FileExtension::Png, "gif" => FileExtension::Gif,
            "bmp" => FileExtension::Bmp, "tiff" => FileExtension::Tiff, "webp" => FileExtension::Webp,
            "svg" => FileExtension::Svg, "ico" => FileExtension::Ico, "psd" => FileExtension::Psd, "ai" => FileExtension::Ai,
            "eps" => FileExtension::Eps, "raw" => FileExtension::Raw,

            "mp3" => FileExtension::Mp3, "wav" => FileExtension::Wav, "ogg" => FileExtension::Ogg, "flac" => FileExtension::Flac,
            "aac" => FileExtension::Aac, "m4a" => FileExtension::M4a, "wma" => FileExtension::Wma, "amr" => FileExtension::Amr,
            "midi" => FileExtension::Midi,

            "mp4" => FileExtension::Mp4, "mkv" => FileExtension::Mkv, "avi" => FileExtension::Avi,
            "mov" => FileExtension::Mov, "wmv" => FileExtension::Wmv, "flv" => FileExtension::Flv,
            "webm" => FileExtension::Webm, "mpeg" => FileExtension::Mpeg, "m4v" => FileExtension::M4v,

            "zip" => FileExtension::Zip, "rar" => FileExtension::Rar, "tar" => FileExtension::Tar,
            "gz" => FileExtension::Gz, "bz2" => FileExtension::Bz2, "7z" => FileExtension::SevenZ,
            "iso" => FileExtension::Iso, "dmg" => FileExtension::Dmg, "xz" => FileExtension::Xz, "lz" => FileExtension::Lz,
            "cab" => FileExtension::Cab, "pak" => FileExtension::Pak,

            "exe" => FileExtension::Exe, "dll" => FileExtension::Dll, "sh" => FileExtension::Sh,
            "bat" => FileExtension::Bat, "cmd" => FileExtension::Cmd, "appimage" => FileExtension::AppImage,
            "jar" => FileExtension::Jar, "bin" => FileExtension::Bin, "run" => FileExtension::Run, "msi" => FileExtension::Msi,
            "elf" => FileExtension::Elf,

            "html" => FileExtension::Html, "css" => FileExtension::Css, "jsx" => FileExtension::Jsx,
            "tsx" => FileExtension::Tsx, "php" => FileExtension::Php, "asp" => FileExtension::Asp, "aspx" => FileExtension::Aspx,

            "sql" => FileExtension::Sql, "db" => FileExtension::Db, "sqlite" => FileExtension::Sqlite,
            "mdb" => FileExtension::Mdb, "accdb" => FileExtension::Accdb, "jsondb" => FileExtension::JsonDb,

            "vmdk" => FileExtension::Vmdk, "vdi" => FileExtension::Vdi, "vhd" => FileExtension::Vhd,
            "vhdx" => FileExtension::Vhdx, "qcow2" => FileExtension::Qcow2,

            "ttf" => FileExtension::Ttf, "otf" => FileExtension::Otf, "woff" => FileExtension::Woff, "woff2" => FileExtension::Woff2,
            "eot" => FileExtension::Eot,

            "obj" => FileExtension::Obj, "fbx" => FileExtension::Fbx, "stl" => FileExtension::Stl, "blend" => FileExtension::Blend,
            "glb" => FileExtension::Glb, "gltf" => FileExtension::Gltf, "ply" => FileExtension::Ply,

            "wallet" => FileExtension::Wallet, "key" => FileExtension::Key, "jsonkey" => FileExtension::JsonKey,
            "pem" => FileExtension::Pem, "p12" => FileExtension::P12,

            _ => FileExtension::Unknown,
        }
    }
}