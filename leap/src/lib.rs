use std::ops::Rem;

pub fn is_leap_year(year: u64) -> bool {
    // todo!("true if {year} is a leap year")
    year.rem(4) == 0 && (year.rem(100) != 0 || year.rem(400) == 0)
}
