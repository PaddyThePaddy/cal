use regex::{Captures, Regex};

lazy_static! {
  static ref BIT_REGEX: Regex = Regex::new(r"(?i)BIT(\d+)").unwrap();
  static ref KB_REGEX: Regex = Regex::new(r"(?i)(\d+)KB").unwrap();
  static ref MB_REGEX: Regex = Regex::new(r"(?i)(\d+)MB").unwrap();
  static ref GB_REGEX: Regex = Regex::new(r"(?i)(\d+)GB").unwrap();
  static ref TB_REGEX: Regex = Regex::new(r"(?i)(\d+)TB").unwrap();
  static ref PB_REGEX: Regex = Regex::new(r"(?i)(\d+)PB").unwrap();
  static ref HEX_REGEX1: Regex = Regex::new(r"(?i)0x([a-f0-9]+)").unwrap();
  static ref HEX_REGEX2: Regex = Regex::new(r"(?i)([a-f0-9]+)(?-i)h").unwrap();
  static ref BIN_REGEX1: Regex = Regex::new(r"(?i)0b([01]+)").unwrap();
  static ref BIN_REGEX2: Regex = Regex::new(r"(?i)([01]+)(?-i)b").unwrap();
  static ref OCT_REGEX: Regex = Regex::new(r"(?i)\b0([0-7]+)\b").unwrap();
  static ref BIT_FN_REGEX: Regex = Regex::new(r"(?i)\b(or|xor|and|not)\b").unwrap();
}

pub fn pre_process(input: &str /* , vars: &HashMap<String, String>*/) -> String {
  let mut result: String;
  result = BIT_FN_REGEX.replace(input, "bit$1").into();
  result = BIT_REGEX.replace(&result, "shl(1, $1)").into();
  result = KB_REGEX.replace(&result, "($1 * 1024)").into();
  result = MB_REGEX.replace(&result, "($1 * 1024 * 1024)").into();
  result = GB_REGEX
    .replace(&result, "($1 * 1024 * 1024 * 1024)")
    .into();
  result = TB_REGEX
    .replace(&result, "($1 * 1024 * 1024 * 1024 * 1024)")
    .into();
  result = PB_REGEX
    .replace(&result, "($1 * 1024 * 1024 * 1024 * 1024 * 1024)")
    .into();

  result = HEX_REGEX1
    .replace(&result, |cap: &Captures| {
      u128::from_str_radix(cap.get(1).unwrap().as_str(), 16)
        .unwrap()
        .to_string()
    })
    .into_owned();

  result = HEX_REGEX2
    .replace(&result, |cap: &Captures| {
      u128::from_str_radix(cap.get(1).unwrap().as_str(), 16)
        .unwrap()
        .to_string()
    })
    .into_owned();

  result = BIN_REGEX1
    .replace(&result, |cap: &Captures| {
      u128::from_str_radix(cap.get(1).unwrap().as_str(), 2)
        .unwrap()
        .to_string()
    })
    .into_owned();

  result = BIN_REGEX2
    .replace(&result, |cap: &Captures| {
      u128::from_str_radix(cap.get(1).unwrap().as_str(), 2)
        .unwrap()
        .to_string()
    })
    .into_owned();

  result = OCT_REGEX
    .replace(&result, |cap: &Captures| {
      u128::from_str_radix(cap.get(1).unwrap().as_str(), 8)
        .unwrap()
        .to_string()
    })
    .into_owned();
  // vars.iter().for_each(|(key, val)| {
  //     result = result.replace(key, val);
  // });
  return result;
}
