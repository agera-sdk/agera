/*!
Work with URIs.
*/

use super::regexp::*;

/// Escapes certain URI characters. Escapes all characters except:
/// ```text
/// A–Z a–z 0–9 - _ . ! ~ * ' ( )
/// 
/// ; / ? : @ & = + $ , #
/// ```
pub fn encode_uri(string: &str) -> String {
    regexp_replace_all!(r"[^A-Za-z0-9\-_\.!\~*'();/?:@&=+&,\#]", string, |seq: &str| {
        String::from_iter(seq.to_owned().bytes().map(|ch| "%".to_owned() + &octet_to_hex(ch)))
    }).into_owned()
}

/// Decodes URIs by unescaping special characters in the form `%XX`.
/// Any invalid character sequences are ignored.
pub fn decode_uri(string: &str) -> String {
    regexp_replace_all!(r"(%[A-Fa-f0-9]{2})+", string, |seq: &str, _| {
        let mut lossy_utf8 = Vec::<u8>::new();
        let mut input = seq.chars();
        while input.next().is_some() {
            lossy_utf8.push(u8::from_str_radix(String::from_iter([input.next().unwrap(), input.next().unwrap()]).as_ref(), 16).unwrap_or(0));
        }
        String::from_utf8_lossy(lossy_utf8.as_ref()).into_owned()
    }).into_owned()
}

/// Escapes certain characters from URI component. Escapes all characters except:
/// ```text
/// A–Z a–z 0–9 - _ . ! ~ * ' ( )
/// ```
pub fn encode_uri_component(string: &str) -> String {
    regexp_replace_all!(r"[^A-Za-z0-9\-_\.!~*'()]", string, |seq: &str| {
        String::from_iter(seq.to_owned().bytes().map(|ch| "%".to_owned() + &octet_to_hex(ch)))
    }).into_owned()
}

/// Decodes URI components by unescaping special characters in the form `%XX`.
/// Any invalid character sequences are ignored.
pub fn decode_uri_component(string: &str) -> String {
    decode_uri(string)
}

fn octet_to_hex(arg: u8) -> String {
    format!("{:02X}", arg)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn escaping() {
        assert_eq!(encode_uri_component(":"), "%3A");
        assert_eq!(decode_uri(encode_uri("\u{10FFFF}").as_ref()), "\u{10FFFF}");
    }
}