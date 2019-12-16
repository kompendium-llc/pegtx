#![allow(dead_code)]
use super::*;

// Convert from pegtoshis to the pAmount
pub fn p_amount(pegtoshis: usize) -> Decimal {
  Decimal::new(pegtoshis as i64, 8)
}

pub fn update_offset(nextoffset: usize, params: &mut TxParams) {
  if nextoffset == 0 {
    params.offset = None
  } else {
    params.offset = Some(nextoffset);
  }
}

pub fn string_to_static_str(s: String) -> &'static str {
  Box::leak(s.into_boxed_str())
}

pub fn csv_to_html(input: &str) -> String {
  let mut s = String::new();
  s += "<table>\n<tr><td>";
  for c in input.chars() {
      match c {
          '\n' =>  s+="</td></tr>\n<tr><td>",
          ','  =>  s+="</td><td>",
          '<'  =>  s+="&lt;",
          '>'  =>  s+="&gt;",
          '&'  =>  s+="&amp;",
          _    =>  s+= &c.to_string()
      }
  }
  s += "</td></tr>\n</table>";
  s
}
