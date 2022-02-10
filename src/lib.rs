#![allow(dead_code)]
//! Library to convert an ASCII string into Hex
//! The general flow goes like this:
//! 1. Convert the ASCII into bytes.
//! 2. Convert the bytes into Hex Strings
//! 3. Concatenate the Hex Strings
//! 4. Return the single Hex String

pub mod ascii_to_hex {

    /// What gets called from outside
    pub fn get_hex_string<'a>(ascii_string: &'a str) -> String {
        let ascii_bytes = get_bytes(ascii_string);
        let hex_vec = convert_to_hex(ascii_bytes);
        concatenate_slices(hex_vec)
    }

    /// This function takes in the ASCII string and
    /// turns it into its byte representation.
    /// This returns a u8 slice.
    fn get_bytes<'a>(ascii_string: &'a str) -> &'a [u8] {
        ascii_string.as_bytes()
    }
    /// This function will convert each u8 slice element
    /// into a String element containing the hex representation
    /// of the u8 element
    fn convert_to_hex<'a>(ascii_bytes: &'a [u8]) -> Vec<String> {
        ascii_bytes
            .into_iter()
            .map(|byte| format!("{:X}", byte).as_str().to_owned())
            .collect()
    }
    /// This function simply concatenates all the hex strings
    fn concatenate_slices<'a>(hex_vec: Vec<String>) -> String {
        hex_vec.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::ascii_to_hex;

    #[test]
    fn it_works() {
        let test_string = "test";
        assert_eq!(ascii_to_hex::get_hex_string(test_string), "74657374");
    }

    #[test]
    fn empty_string() {
        let test_string = "";
        assert_eq!(ascii_to_hex::get_hex_string(test_string), "");
    }

    #[test]
    fn long_string() {
        let test_string = "DSJAKLFHASJKGLF879Q472631987HJKFSDAHFLKSJADHsdahlkfjhdskalfh";
        assert_eq!(ascii_to_hex::get_hex_string(test_string), "44534A414B4C464841534A4B474C4638373951343732363331393837484A4B4653444148464C4B534A414448736461686C6B666A6864736B616C6668");
    }

    #[test]
    fn special_characters() {
        let test_string = "!@#%";
        assert_eq!(ascii_to_hex::get_hex_string(test_string), "21402325");
    }
}
