use regex::Regex;
use std::hash::Hash;

/// The parts from an formatted fractional number string.
/// Can represent e.g. "1", "3.14", or "-14.141"
struct FractionalNumberParts {
    // if it has "-" sign
    has_sign: bool,
    whole_part: String,
    fractional_part: Option<String>,
}


/// Splits a formatted fractional number into its parts.
/// We expect only valid values at this point.
fn get_fractional_number_parts(formatted_fractional_num: &str) -> FractionalNumberParts {
    let regex = Regex::new("(?P<sign>-)?(?P<whole_part>[0-9]+)(.(?P<fraction_part>[0-9]+))?").unwrap();
    let captures = regex.captures(formatted_fractional_num).unwrap();
    FractionalNumberParts {
        has_sign: captures.name("sign").is_some(),
        whole_part: captures.name("whole_part").map(|x| x.as_str().to_owned()).unwrap(),
        fractional_part: captures.name("fraction_part").map(|x| x.as_str().to_owned()),
    }
}

/// Aligns two fraction numbers using "." as sign/separator for the fraction part.
/// This is done via additional spaces. Afterwards the values can be right-aligned
/// but digits/places match in both.
/// Takes into account that both may have fraction parts of multiple lengths.
pub fn fmt_align_fraction_strings(fraction1: &str, fraction2: &str) -> (String, String) {
    let mut parts_frac_1 = get_fractional_number_parts(fraction1);
    let mut parts_frac_2 = get_fractional_number_parts(fraction2);

    // destruct / de-own values
    let FractionalNumberParts {
        has_sign: f1_has_sign,
        whole_part: f1_whole_part,
        fractional_part: mut f1_fractional_part
    } = parts_frac_1;
    // destruct / de-own values
    let FractionalNumberParts {
        has_sign: f2_has_sign,
        whole_part: f2_whole_part,
        fractional_part: mut f2_fractional_part
    } = parts_frac_2;

    f1_fractional_part = normalized_fractional_part_or_none(f1_fractional_part);
    f2_fractional_part = normalized_fractional_part_or_none(f2_fractional_part);

    let (mut first, mut second) = (String::new(), String::new());

    // add additional space for sign if only one number has it
    if f1_has_sign && !f2_has_sign {
        second.push(' ');
    } else if !f1_has_sign && f2_has_sign {
        first.push(' ');
    }

    // NO this is done by my generic "alight right" mechanism
    // add additional spaces for the whole part
    /*if f1_whole_part.len() > f2_whole_part.len() {
        second.push_str(&format!("{:1$}", ' ', f1_whole_part.len() - f2_whole_part.len()));
    }
    else if f2_whole_part.len() > f1_whole_part.len() {
        first.push_str(&format!("{:1$}", ' ', f2_whole_part.len() - f1_whole_part.len()));
    }*/

    // add sign
    if f1_has_sign {
        first.push('-');
    }
    if f2_has_sign {
        second.push('-');
    }
    // push whole part
    first.push_str(&f1_whole_part);
    second.push_str(&f2_whole_part);
    // add fractional part
    if let Some(val) = f1_fractional_part.as_ref() {
        first.push('.');
        first.push_str(val);
    }
    if let Some(val) = f2_fractional_part.as_ref() {
        second.push('.');
        second.push_str(val);
    }

    // add spaces at the end so that both
    // fractional parts match at the border between
    // whole part and fractional part
    // TODO how is this called in english?
    // Beide sollen an der Kommastelle aligned sein

    // f1_fractional_part is None or (Some(>0)), therefore we can do this
    let f1_fractional_len = f1_fractional_part.as_ref().map(|x| x.len()).unwrap_or(0);
    let f2_fractional_len = f2_fractional_part.as_ref().map(|x| x.len()).unwrap_or(0);

    // add additional space for '.' if only one has it
    if f1_fractional_len == 0 && f2_fractional_len > 0 {
        first.push(' ');
    }
    else if f1_fractional_len > 0 && f2_fractional_len == 0 {
        second.push(' ');
    }

    // add spaces at the end to align to the others fractional part
    if f1_fractional_len > f2_fractional_len {
        let len = f1_fractional_len - f2_fractional_len;
        second.push_str(&format!("{:1$}", ' ', len));
    }
    else if f2_fractional_len > f1_fractional_len {
        let len = f2_fractional_len - f1_fractional_len;
        first.push_str(&format!("{:1$}", ' ', len));
    }

    (first, second)
}

/// Takes the fractional part, removes all zeroes and afterwards returns
/// the new fractional part string. If after the removing of the zeroes
/// only "" is left, than None get's returned.
fn normalized_fractional_part_or_none(mut fp: Option<String>) -> Option<String> {
    fp = fp.map(|x| fractional_part_remove_zeroes(&x));
    if fp.is_some() && fp.as_ref().unwrap().is_empty() {
        None
    } else { fp }
}

/// Tells that in "123000" (fractional part of "0.123000") and returns "123"
fn fractional_part_remove_zeroes(fractional_part: &str) -> String {
    let zeroes = fractional_part_count_zeroes(fractional_part);
    let slice = &fractional_part[0..fractional_part.len() - zeroes];
    slice.to_string()
}

/// Tells that in "123000" (fractional part of "0.123000") are three unnecessary zeroes.
fn fractional_part_count_zeroes(fractional_part: &str) -> usize {
    let mut zeroes = 0;
    let chars = fractional_part.chars().collect::<Vec<char>>();
    for i in 0..fractional_part.len() {
        // go backwards
        let i = fractional_part.len() - 1 - i;
        let char = chars[i];
        if char == '0' {
            zeroes += 1;
        }
    }
    zeroes
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_fractional_number_parts() {
        let num = -13.37_f64;
        let num_fmt = format!("{:.2}", num);
        let parts = get_fractional_number_parts(&num_fmt);
        assert_eq!(true, parts.has_sign);
        assert_eq!("13", parts.whole_part);
        assert_eq!("37", parts.fractional_part.unwrap());


        let num = 1411010_f64;
        let num_fmt = format!("{}", num);
        let parts = get_fractional_number_parts(&num_fmt);
        assert_eq!(false, parts.has_sign);
        assert_eq!("1411010", parts.whole_part);
        assert_eq!(true, parts.fractional_part.is_none());
    }

    #[test]
    fn test_fractional_part_count_zeroes() {
        assert_eq!(3, fractional_part_count_zeroes("123000"));
        assert_eq!(0, fractional_part_count_zeroes("123"));
        assert_eq!(1, fractional_part_count_zeroes("0"));
    }

    #[test]
    fn test_fractional_part_remove_zeroes() {
        assert_eq!("123", fractional_part_remove_zeroes("123000"));
        assert_eq!("123", fractional_part_remove_zeroes("123"));
        assert_eq!("", fractional_part_remove_zeroes("0"));
    }

    #[test]
    fn test_fractional_part_or_none() {
        assert_eq!(Some("123".to_owned()), normalized_fractional_part_or_none(Some("123000".to_owned())));
        assert_eq!(Some("123".to_owned()), normalized_fractional_part_or_none(Some("123".to_owned())));
        assert_eq!(None, normalized_fractional_part_or_none(Some("0".to_owned())));
        assert_eq!(None, normalized_fractional_part_or_none(Some("00000".to_owned())));
        assert_eq!(None, normalized_fractional_part_or_none(None));
    }

    #[test]
    fn test_fmt_align_fraction_strings() {
        assert_eq!(("2".to_owned(), " 2.5".to_owned()), fmt_align_fraction_strings("2", "2.5"));
        assert_eq!(("2.123".to_owned(), "2.13".to_owned()), fmt_align_fraction_strings("2.123", "2.12"));
    }

}
