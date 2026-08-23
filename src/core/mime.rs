/// Content type for an attachment, guessed from its filename.
///
/// Atlassian stores whatever content type the upload declares, and it decides whether a browser previews the file or
/// offers it as a download. Sending `application/octet-stream` for everything turns every screenshot into an
/// anonymous blob, which is why this exists.
///
/// The table is deliberately short: the formats people actually attach to issues. An unknown extension falls back to
/// `application/octet-stream`, which is what the upload would have said anyway.
const MIME_TYPES: &[(&str, &str)] = &[
    ("png", "image/png"),
    ("jpg", "image/jpeg"),
    ("jpeg", "image/jpeg"),
    ("gif", "image/gif"),
    ("webp", "image/webp"),
    ("svg", "image/svg+xml"),
    ("bmp", "image/bmp"),
    ("ico", "image/vnd.microsoft.icon"),
    ("tif", "image/tiff"),
    ("tiff", "image/tiff"),
    ("avif", "image/avif"),
    ("heic", "image/heic"),
    ("pdf", "application/pdf"),
    ("doc", "application/msword"),
    (
        "docx",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    ),
    ("xls", "application/vnd.ms-excel"),
    (
        "xlsx",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    ),
    ("ppt", "application/vnd.ms-powerpoint"),
    (
        "pptx",
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    ),
    ("odt", "application/vnd.oasis.opendocument.text"),
    ("ods", "application/vnd.oasis.opendocument.spreadsheet"),
    ("rtf", "application/rtf"),
    ("txt", "text/plain"),
    ("md", "text/markdown"),
    ("csv", "text/csv"),
    ("tsv", "text/tab-separated-values"),
    ("log", "text/plain"),
    ("json", "application/json"),
    ("xml", "application/xml"),
    ("yaml", "application/yaml"),
    ("yml", "application/yaml"),
    ("html", "text/html"),
    ("htm", "text/html"),
    ("css", "text/css"),
    ("js", "text/javascript"),
    ("mjs", "text/javascript"),
    ("ts", "text/plain"),
    ("sql", "application/sql"),
    ("zip", "application/zip"),
    ("gz", "application/gzip"),
    ("tar", "application/x-tar"),
    ("7z", "application/x-7z-compressed"),
    ("rar", "application/vnd.rar"),
    ("bz2", "application/x-bzip2"),
    ("mp4", "video/mp4"),
    ("webm", "video/webm"),
    ("mov", "video/quicktime"),
    ("avi", "video/x-msvideo"),
    ("mp3", "audio/mpeg"),
    ("wav", "audio/wav"),
    ("ogg", "audio/ogg"),
    ("m4a", "audio/mp4"),
];

pub const DEFAULT_MIME_TYPE: &str = "application/octet-stream";

/// The content type for `filename`, or `application/octet-stream` when the extension is unknown or absent.
///
/// A leading dot is not an extension — `.gitignore` is a file called `.gitignore`, not a `gitignore` file.
pub fn mime_type_for(filename: &str) -> &'static str {
    let Some(dot) = filename.rfind('.') else {
        return DEFAULT_MIME_TYPE;
    };

    if dot == 0 {
        return DEFAULT_MIME_TYPE;
    }

    let extension = filename[dot + 1..].to_lowercase();

    MIME_TYPES
        .iter()
        .find(|(name, _)| *name == extension)
        .map_or(DEFAULT_MIME_TYPE, |(_, mime)| *mime)
}
