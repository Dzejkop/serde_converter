use anyhow::Error;
use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;
use std::{convert::TryFrom, fmt};

const COMPRESSION_LEVEL: u8 = 5;

/// Encapsulates a string, compressed, base64'ed and urlencoded
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PayloadString(String);

impl PayloadString {
    pub fn from_encoded(s: String) -> Self {
        Self(s)
    }
}

impl From<String> for PayloadString {
    fn from(value: String) -> Self {
        Self(urlencoding::encode(&base64::encode(&compress_to_vec(
            value.as_bytes(),
            COMPRESSION_LEVEL,
        ))))
    }
}

impl TryFrom<PayloadString> for String {
    type Error = Error;

    fn try_from(value: PayloadString) -> Result<Self, Self::Error> {
        let decoded = urlencoding::decode(&value.0)?;
        let bytes = &base64::decode(decoded)?;

        let decompressed = decompress_to_vec(bytes).map_err(|err| {
            Error::msg(format!(
                "Decompression failed with status code {:?}",
                err
            ))
        })?;

        Ok(String::from_utf8(decompressed)?)
    }
}

impl fmt::Display for PayloadString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use test_case::test_case;

    #[test_case("ASDFASFDSAFS")]
    #[test_case(")0000000000000fdsa<>BCVX;l1kiop32[i56")]
    #[test_case(indoc! {r#"
        {
            "name": "Me",
            "age": 23
        }
    "#})]
    fn convert(s: &str) {
        let payload = PayloadString::from(s.to_string());

        let actual = String::try_from(payload).unwrap();

        assert_eq!(s, &actual);
    }

    #[test_case("AAAA", "c3R0dAQA")]
    #[test_case("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA", "c3QkFgAA")]
    #[test_case("afdsaf7as7w907rv8n947tv957s07tm890v5estv789e80v7sm8s079", "DckxDoAwDAPAL2WgcvycSLRbGXBkvk%2Fnq3WrFkr4GHidDy%2B0OaBA72R4TLWRnBmGdh7gDw%3D%3D")]
    #[test_case(indoc! {r#"{
            "name": "Me",
            "age": 23
        }
    "#}, "q%2BZSAAKlvMTcVCUrBSXfVCUdiEhiOkjAyJirlgsA")]
    fn display(s: &str, exp: &str) {
        let payload = PayloadString::from(s.to_string());

        let payload = payload.to_string();

        assert_eq!(exp, payload.as_str());
    }
}
