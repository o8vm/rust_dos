use alloc::vec::Vec;
use core::slice;

fn command_tail<'a>() -> &'a [u8] {
    let ptr = 0x80 as *const u8;
    let (len, bytes) = unsafe {
        let len = *ptr as usize;
        (len, slice::from_raw_parts(ptr.add(1), len))
    };
    let trimmed = bytes.iter()
        .position(|&b| b == 0x0D)
        .unwrap_or(len);
    &bytes[..trimmed]
}

/// Returns the command line arguments as a vector of string slices.
/// Handles quoted arguments and whitespace separation.
///
/// # Warning
/// Unlike standard operating system, the first argument (index 0) is not the program name,
/// but the first argument after the program name.
pub fn args() -> Vec<&'static str> {
    let mut args = Vec::new();
    let line = command_tail();

    let s = unsafe { core::str::from_utf8_unchecked(line) };
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= len { break; }

        let start = i;
        let mut in_quotes = false;

        while i < len {
            let c = bytes[i];
            if c == b'"' {
                in_quotes = !in_quotes;
                i += 1;
                continue;
            }
            if !in_quotes && c.is_ascii_whitespace() {
                break;
            }
            i += 1;
        }

        let end = i;
        let token = s[start..end].trim_matches('"');
        args.push(token);
    }

    args
}