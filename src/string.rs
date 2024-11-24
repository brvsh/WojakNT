use winapi::shared::ntdef::UNICODE_STRING;

pub fn new_unicode(s: &[u16]) -> UNICODE_STRING {
    let l = s.len();

    let n = if l > 0 && s[l - 1] == 0 { l - 1 } else { l };

    UNICODE_STRING {
        Length: (n * 2) as u16,
        MaximumLength: (l * 2) as u16,
        Buffer: s.as_ptr() as _,
    }
}
