use log::error;
use std::error::Error;
use std::fmt;

use crate::case;

use crate::work_hour::WorkHour;

#[derive(Debug)]
pub enum ValidError {
    NotEqualsTotalHourAndCasesTotalHourError,
    TotalPercentIsntOneError,
}
impl Error for ValidError {}

impl fmt::Display for ValidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidError::NotEqualsTotalHourAndCasesTotalHourError => {
                write!(f, "NotEqualsTotalHourAndCasesTotalHourError")
            }
            ValidError::TotalPercentIsntOneError => {
                write!(f, "TotalPercentIsntOneError")
            }
        }
    }
}

pub fn valid_total_percent(cases: &Vec<case::Case>) -> Result<(), Box<dyn Error>> {
    let total_percent = cases.iter().fold(0.0, |sum, p| sum + p.percent());

    if 0.9999 <= total_percent && total_percent <= 1.0001 {
        Ok(())
    } else {
        error!("各案件のパーセントの合計が1ではない: {}", total_percent);
        Err(Box::new(ValidError::TotalPercentIsntOneError))
    }
}

pub fn valid_equals_total_hour_and_cases_total_hour(
    total_hour: &WorkHour,
    cases: &Vec<case::Case>,
) -> Result<(), Box<dyn Error>> {
    let case_total_hour = cases
        .iter()
        .fold(WorkHour::new(0.0), |sum, p| sum + p.total_hour());
    if *total_hour == case_total_hour {
        Ok(())
    } else {
        error!(
            "計算結果と合計時間が不一致: {} != {}",
            total_hour, case_total_hour
        );
        Err(Box::new(
            ValidError::NotEqualsTotalHourAndCasesTotalHourError,
        ))
    }
}
