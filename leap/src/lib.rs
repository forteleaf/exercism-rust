use std::ops::Rem;

pub fn is_leap_year(year: u64) -> bool {
    // todo!("true if {year} is a leap year")
    if year.rem(4) == 0 {
        if year.rem(400) == 0 {
            true
        } else {
            year.rem(100) != 0
        }
    } else {
        false
    }
}
