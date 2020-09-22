pub fn is_leap_year(year: u64) -> bool {
    if year % 400 == 0 {
        println!("check 1");
        true
    } else if year % 100 == 0 {
        println!("check 2");
        false
    } else if year % 4 == 0 {
        println!("check 3");
        true
    } else {
        println!("no ifs");
        false
    }
}
