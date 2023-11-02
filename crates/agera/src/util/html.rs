/// Escapes HTML special characters.
pub fn escape_html(characters: &str) -> String {
    let escaped = htmlentity::entity::encode(characters.as_ref(), &htmlentity::entity::EncodeType::NamedOrHex, &htmlentity::entity::CharacterSet::SpecialChars);
    String::from_utf8_lossy(&escaped.bytes().into_owned()).into_owned()
}

/// Decodes HTML entities from HTML text.
pub fn unescape_html(characters: &str) -> String {
    let unescaped = htmlentity::entity::decode(characters.as_ref());
    String::from_utf8_lossy(&unescaped.bytes().into_owned()).into_owned()
}