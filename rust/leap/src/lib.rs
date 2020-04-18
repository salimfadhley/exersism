use std::ops::Rem;

pub fn is_leap_year(year: u64) -> bool {
    let db: fn(u64, u64) -> bool = |n: u64, r: u64| n.rem(r) == 0;
    db(year, 4) && (db(year, 400) | !db(year, 100))
}
