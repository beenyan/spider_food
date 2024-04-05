pub mod args;
pub mod deserialize;
pub mod serialize;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TIME_RANGE1: Regex = Regex::new(r"\d{1,2}\s*[:|：|-|~]\s*\d{1,2}").unwrap();
    static ref TIME_RANGE2: Regex = Regex::new(r"\d{4}\s*[-|~]\s*\d{4}").unwrap();
    static ref WEEKDAY: Regex = Regex::new(r"^(週|星期)(一|二|三|四|五|六|日)").unwrap();
    static ref TIME_KEY: Regex = Regex::new(r"(平日|六日|假日|時間)").unwrap();
    static ref TIMES: [Regex; 4] = [
        TIME_RANGE1.clone(),
        TIME_RANGE2.clone(),
        WEEKDAY.clone(),
        TIME_KEY.clone()
    ];
}

pub fn is_time(str: &str) -> bool {
    TIMES.iter().any(|regex| regex.is_match(str))
}
