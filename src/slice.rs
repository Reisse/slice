use std::cmp;
use std::collections::VecDeque;
use std::fmt;
use std::io::{self, BufRead, Write};

#[derive(Debug, PartialEq)]
pub struct Slice {
    pub begin: isize,
    pub end: Option<isize>
}

impl Slice {
    pub fn from_string(slice_str: &str) -> Result<Slice, &'static str> {
        let parts: Vec<&str> = slice_str.split(':').collect();

        if parts.len() != 2 {
            return Err("Invalid slice");
        }
        if parts[0].is_empty() && parts[1].is_empty() {
            return Err("Slice cannot be empty");
        }

        let mut slice = Slice{ begin: 0, end: None };
        if !parts[0].is_empty() {
            match parts[0].parse::<isize>() {
                Ok(begin_value) => slice.begin = begin_value,
                Err(_) => return Err("Invalid slice starting point")
            }
        }
        if !parts[1].is_empty() {
            match parts[1].parse::<isize>() {
                Ok(end_value) => slice.end = Some(end_value),
                Err(_) => return Err("Invalid slice ending point")
            }
        }
        return Ok(slice);
    }
}

impl fmt::Display for Slice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.end {
            Some(end_v) => write!(f, "[{}:{}]", self.begin, end_v),
            None => write!(f, "[{}:]", self.begin)
        }
    }
}

pub fn slice_input(slice: Slice, input: &mut dyn BufRead, output: &mut dyn Write) -> io::Result<()> {
    enum PrintMode { Buf, Overflow };
    let mut mode = PrintMode::Overflow;

    let mut buf_size: usize = 0;

    let mut skip_count: usize = 0;
    let mut stop_count: usize = std::usize::MAX;

    if slice.begin > 0 {
        skip_count = slice.begin as usize;
    }

    if let Some(slice_end) = slice.end {
        if slice_end >= 0 {
            stop_count = (slice_end as usize).saturating_sub(skip_count);
        } else {
            buf_size = -slice_end as usize;
        }
    }

    if slice.begin < 0 {
        mode = PrintMode::Buf;
        buf_size = -slice.begin as usize;
    }

    let mut buf: VecDeque<String> = VecDeque::new();
    buf.reserve(buf_size);

    let mut lines_processed: usize = 0;
    for maybe_line in input.lines().skip(skip_count).take(stop_count.saturating_add(buf_size)) {
        let line = maybe_line?;

        if buf_size == 0 {
            writeln!(output, "{}", line)?;
        } else {
            if buf.len() == buf_size {
                let front = buf.pop_front().unwrap();
                if let PrintMode::Overflow = mode {
                    writeln!(output, "{}", front)?;
                }
            }

            buf.push_back(line);
        }

        lines_processed += 1;
    }

    if let PrintMode::Buf = mode {
        if let Some(slice_end) = slice.end {
            if slice_end < 0 {
                buf.truncate(buf.len() - cmp::min(buf.len(), -slice_end as usize));
            } else {
                buf.truncate(
                    buf.len() - cmp::min(buf.len(), lines_processed.saturating_sub(stop_count)));
            }
        }

        for line in buf {
            writeln!(output, "{}", line)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::iter;

    #[test]
    fn slice_from_string_empty() {
        assert!(Slice::from_string("").is_err());
    }

    #[test]
    fn slice_from_string_letters() {
        assert!(Slice::from_string("abc").is_err());
    }

    #[test]
    fn slice_from_string_single_number() {
        assert!(Slice::from_string("1").is_err());
    }

    #[test]
    fn slice_from_string_two_numbers_colon() {
        assert!(Slice::from_string("1:2:").is_err());
    }

    #[test]
    fn slice_from_string_colon_two_numbers() {
        assert!(Slice::from_string(":1:2").is_err());
    }

    #[test]
    fn slice_from_string_invalid_begin() {
        assert!(Slice::from_string("a:2").is_err());
    }

    #[test]
    fn slice_from_string_invalid_end() {
        assert!(Slice::from_string("1:b").is_err());
    }

    #[test]
    fn slice_from_string_colon_three_numbers() {
        assert!(Slice::from_string("1:2:3").is_err());
    }

    #[test]
    fn slice_from_string_colon() {
        assert!(Slice::from_string(":").is_err());
    }

    #[test]
    fn slice_from_string_two_numbers() {
        assert_eq!(Slice::from_string("1:2"), Ok(Slice{ begin: 1, end: Some(2) }));
    }

    #[test]
    fn slice_from_string_number_colon() {
        assert_eq!(Slice::from_string("1:"), Ok(Slice{ begin: 1, end: None }));
    }

    #[test]
    fn slice_from_string_colon_number() {
        assert_eq!(Slice::from_string(":1"), Ok(Slice{ begin: 0, end: Some(1) }));
    }

    #[test]
    fn slice_from_string_negative_number() {
        assert_eq!(Slice::from_string("-1:2"), Ok(Slice{ begin: -1, end: Some(2) }));
    }

    #[test]
    fn slice_from_string_number_negative() {
        assert_eq!(Slice::from_string("1:-2"), Ok(Slice{ begin: 1, end: Some(-2) }));
    }

    #[test]
    fn slice_from_string_two_negatives() {
        assert_eq!(Slice::from_string("-1:-2"), Ok(Slice{ begin: -1, end: Some(-2) }));
    }

    #[test]
    fn slice_display() {
        assert_eq!(format!("{}", Slice{ begin: 1, end: Some(2) }), "[1:2]");
    }

    #[test]
    fn slice_display_end_none() {
        assert_eq!(format!("{}", Slice{ begin: 1, end: None }), "[1:]");
    }

    // Poor man's parametrized tests
    const TEST_INPUTS: [&'static str; 6] = [
        "abc def\ndef ghi\nghi jkl\njkl mno\nmno qwe\n",              // simple text
        "abc\n\n\n\nmno\n",                                           // text with empty lines
        "a\nb\nc\nd\ne",                                              // no newline at end of text
        // same inputs, but with \r\n as a line break
        "abc def\r\ndef ghi\r\nghi jkl\r\njkl mno\r\nmno qwe\r\n",    // simple text
        "abc\r\n\r\n\r\n\r\nmno\r\n",                                 // text with empty lines
        "a\r\nb\r\nc\r\nd\r\ne",                                      // no newline at end of text
    ];

    fn check_slice(slice_str: &str, input: &str, expected_output: &str) {
        let slice = Slice::from_string(slice_str).unwrap();
        let mut in_buf = input.as_bytes();
        let mut out_buf = Vec::new();
        assert!(slice_input(slice, &mut in_buf, &mut out_buf).is_ok());

        let output = std::str::from_utf8(&out_buf).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test] // lines > begin >= 0, end is absent
    fn slice_begin_pos_end_none() {
        for input in TEST_INPUTS.iter() {
            let expected_output =
                    input
                        .lines()
                        .skip(1)
                        .flat_map(|s| s.chars().chain(iter::once('\n')))
                        .collect::<String>();
            check_slice("1:", input, &expected_output);
        }
    }

    #[test] // lines > begin >= 0, end < 0, |end| >= lines - begin
    fn slice_begin_pos_end_neg_abs_end_ge_lines_minus_begin() {
        for input in TEST_INPUTS.iter() {
            check_slice("2:-4", input, "");
        }
    }

    #[test] // lines > begin >= 0, end < 0, |end| < lines - begin
    fn slice_begin_pos_end_neg_abs_end_lt_lines_minus_begin() {
        for input in TEST_INPUTS.iter() {
            let expected_output =
                    input
                        .lines()
                        .skip(2)
                        .take(2)
                        .flat_map(|s| s.chars().chain(iter::once('\n')))
                        .collect::<String>();
            check_slice("2:-1", input, &expected_output)
        }
    }

    #[test] // lines > begin >= 0, end > begin > 0
    fn slice_begin_pos_end_pos_end_gt_begin() {
        for input in TEST_INPUTS.iter() {
            let expected_output =
                    input
                        .lines()
                        .skip(2)
                        .take(2)
                        .flat_map(|s| s.chars().chain(iter::once('\n')))
                        .collect::<String>();
            check_slice("2:4", input, &expected_output)
        }
    }

    #[test] // lines > begin >= 0, begin >= end >= 0
    fn slice_begin_pos_end_pos_end_le_begin() {
        for input in TEST_INPUTS.iter() {
            check_slice("2:1", input, "");
        }
    }

    #[test] // -lines < begin < 0, end is absent
    fn slice_begin_neg_end_none() {
        for input in TEST_INPUTS.iter() {
            let expected_output =
                input
                    .lines()
                    .skip(4)
                    .flat_map(|s| s.chars().chain(iter::once('\n')))
                    .collect::<String>();
            check_slice("-1:", input, &expected_output);
        }
    }

    #[test] // -lines < begin < 0, end < 0, |end| >= |begin|
    fn slice_begin_neg_end_neg_abs_end_ge_abs_begin() {
        for input in TEST_INPUTS.iter() {
            check_slice("-1:-2", input, "");
        }
    }

    #[test] // -lines < begin < 0, end < 0, |end| < |begin|
    fn slice_begin_neg_end_neg_abs_end_lt_abs_begin() {
        for input in TEST_INPUTS.iter() {
            let expected_output =
                input
                    .lines()
                    .take(2)
                    .flat_map(|s| s.chars().chain(iter::once('\n')))
                    .collect::<String>();
            check_slice("-5:-3", input, &expected_output);
        }
    }

    #[test] // -lines < begin < 0, end > 0, end > lines + begin
    fn slice_begin_neg_end_pos_end_gt_lines_plus_begin() {
        for input in TEST_INPUTS.iter() {
            let expected_output =
                input
                    .lines()
                    .skip(1)
                    .take(2)
                    .flat_map(|s| s.chars().chain(iter::once('\n')))
                    .collect::<String>();
            check_slice("-4:3", input, &expected_output);
        }
    }

    #[test] // -lines < begin < 0, end > 0, end <= lines + begin, end <= begin
    fn slice_begin_neg_end_pos_end_le_lines_plus_begin_end_le_begin() {
        for input in TEST_INPUTS.iter() {
            check_slice("-4:1", input, "");
        }
    }

    #[test] // -lines < begin < 0, end > 0, end <= lines + begin, end > begin
    fn slice_begin_neg_end_pos_end_le_lines_plus_begin_end_gt_begin() {
        for input in TEST_INPUTS.iter() {
            check_slice("-1:2", input, "");
        }
    }

    #[test] // begin >= lines, end is absent
    fn slice_begin_ge_lines_end_none() {
        for input in TEST_INPUTS.iter() {
            check_slice("5:", input, "");
        }
    }

    #[test] // begin is absent, end < 0, |end| <= lines
    fn slice_begin_none_end_neg_abs_end_le_lines() {
        for input in TEST_INPUTS.iter() {
            check_slice(":-5", input, "");
        }
    }

    #[test] // begin is absent, end == 0
    fn slice_begin_none_end_zero() {
        for input in TEST_INPUTS.iter() {
            check_slice(":0", input, "");
        }
    }
}