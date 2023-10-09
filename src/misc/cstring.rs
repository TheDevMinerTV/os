pub fn fix_zeroterminated_string(data: &mut [char]) {
    let mut first_zero = None;
    for (i, d) in data.iter_mut().enumerate() {
        if *d == '\0' {
            first_zero = Some(i);
        }

        if first_zero.is_some() {
            *d = ' ';
        }
    }
}
