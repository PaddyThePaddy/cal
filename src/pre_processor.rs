use super::*;
use regex::{Captures, Regex};

lazy_static! {
  static ref BIT_REGEX: Regex = Regex::new(r"(?i)BIT(\d+)").unwrap();
  static ref KB_REGEX: Regex = Regex::new(r"(?i)(\d+)KB").unwrap();
  static ref MB_REGEX: Regex = Regex::new(r"(?i)(\d+)MB").unwrap();
  static ref GB_REGEX: Regex = Regex::new(r"(?i)(\d+)GB").unwrap();
  static ref TB_REGEX: Regex = Regex::new(r"(?i)(\d+)TB").unwrap();
  static ref PB_REGEX: Regex = Regex::new(r"(?i)(\d+)PB").unwrap();
  static ref HEX_REGEX1: Regex = Regex::new(r"\b0x(?i)([a-f0-9]+)\b").unwrap();
  static ref HEX_REGEX2: Regex = Regex::new(r"\b(?i)([a-f0-9]+)(?-i)h\b").unwrap();
  static ref BIN_REGEX1: Regex = Regex::new(r"\b0b(?i)([01]+)\b").unwrap();
  static ref BIN_REGEX2: Regex = Regex::new(r"\b(?i)([01]+)(?-i)b\b").unwrap();
  static ref OCT_REGEX1: Regex = Regex::new(r"\b0o(?i)([0-7]+)\b").unwrap();
  static ref OCT_REGEX2: Regex = Regex::new(r"\b(?i)([0-7]+)(?-i)o\b").unwrap();
  static ref BIT_FN_REGEX: Regex = Regex::new(r"(?i)\b(or|xor|and|not)\b").unwrap();
}

pub fn pre_process(input: &str /* , vars: &HashMap<String, String>*/) -> String {
  let mut result: String;
  result = BIT_FN_REGEX.replace_all(input, "bit$1").into();
  result = BIT_REGEX.replace_all(&result, "shl(1, $1)").into();
  result = KB_REGEX.replace_all(&result, "($1 * 1024)").into();
  result = MB_REGEX.replace_all(&result, "($1 * 1024 * 1024)").into();
  result = GB_REGEX
    .replace_all(&result, "($1 * 1024 * 1024 * 1024)")
    .into();
  result = TB_REGEX
    .replace_all(&result, "($1 * 1024 * 1024 * 1024 * 1024)")
    .into();
  result = PB_REGEX
    .replace_all(&result, "($1 * 1024 * 1024 * 1024 * 1024 * 1024)")
    .into();

  result = HEX_REGEX1
    .replace_all(&result, |cap: &Captures| {
      UintType::from_str_radix(cap.get(1).unwrap().as_str(), 16)
        .unwrap()
        .to_string()
    })
    .into_owned();

  result = HEX_REGEX2
    .replace_all(&result, |cap: &Captures| {
      UintType::from_str_radix(cap.get(1).unwrap().as_str(), 16)
        .unwrap()
        .to_string()
    })
    .into_owned();

  result = BIN_REGEX1
    .replace_all(&result, |cap: &Captures| {
      UintType::from_str_radix(cap.get(1).unwrap().as_str(), 2)
        .unwrap()
        .to_string()
    })
    .into_owned();

  result = BIN_REGEX2
    .replace_all(&result, |cap: &Captures| {
      UintType::from_str_radix(cap.get(1).unwrap().as_str(), 2)
        .unwrap()
        .to_string()
    })
    .into_owned();

  result = OCT_REGEX1
    .replace_all(&result, |cap: &Captures| {
      UintType::from_str_radix(cap.get(1).unwrap().as_str(), 8)
        .unwrap()
        .to_string()
    })
    .into_owned();

  result = OCT_REGEX2
    .replace_all(&result, |cap: &Captures| {
      UintType::from_str_radix(cap.get(1).unwrap().as_str(), 8)
        .unwrap()
        .to_string()
    })
    .into_owned();
  // vars.iter().for_each(|(key, val)| {
  //     result = result.replace_all(key, val);
  // });
  return result;
}
