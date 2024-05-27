
pub fn replace_sc_string(s: &str) -> String {
    let s = s
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
        .replace("\\0", "\0")
        .replace("\\'", "\'")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\");
    s
}
