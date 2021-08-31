use log::{debug, error};
use std::error::Error;
use std::fmt;

use crate::work_hour::WorkHour;

#[derive(Debug)]
pub enum InputError {
    LessThanMinimumWorkHoursError,
    NotQuarterHourIncrementsError(f64),
    WorkDaysOverOneMonthError(u8),
}
impl Error for InputError {}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputError::LessThanMinimumWorkHoursError => write!(f, "LessThanMinimumWorkHoursError"),
            InputError::NotQuarterHourIncrementsError(i) => {
                write!(f, "NotQuarterHourIncrements: {}", i)
            }
            InputError::WorkDaysOverOneMonthError(i) => {
                write!(f, "WorkDaysOverOneMonthError: {}", i)
            }
        }
    }
}

pub fn get_total_hour(
    matches: Option<&str>,
    setting: &Option<String>,
) -> Result<WorkHour, Box<dyn Error>> {
    // Total hour
    let raw_total_hour = match matches {
        Some(hour) => hour,
        None => {
            match setting {
                Some(hour) => hour,
                None => "140.0", // WIP: 標準入力
            }
        }
    };

    let total_hour = match raw_total_hour.parse() {
        Ok(hour) => hour,
        Err(e) => {
            error!("合計時間が誤っています。");
            error!("数字であることを確認してください: {}.", &raw_total_hour);
            return Err(Box::new(e));
        }
    };

    let total_hour = WorkHour::new(total_hour);
    debug!("total_hour({}): {:?}", &total_hour, &total_hour);
    if 140. > total_hour.raw() {
        error!("一人月の労働時間は140時間以上にしてください。");
        Err(InputError::LessThanMinimumWorkHoursError)?;
    } else if 0. < total_hour.reminder() {
        error!(
            "{:.2}時間余分です。労働時間は15分刻みで入力してください。",
            total_hour.reminder()
        );
        Err(InputError::NotQuarterHourIncrementsError(
            total_hour.reminder(),
        ))?;
    }

    Ok(total_hour)
}

pub fn get_work_days(
    matches: Option<&str>,
    setting: &Option<String>,
) -> Result<u8, Box<dyn Error>> {
    // Work days
    let raw_work_days = match matches {
        Some(hour) => hour,
        None => {
            match setting {
                Some(hour) => hour,
                None => "18", // WIP: 標準入力
            }
        }
    };

    let work_days = match raw_work_days.parse() {
        Ok(days) => days,
        Err(e) => {
            error!("労働日数が誤っています。");
            error!("数字であることを確認してください: {}.", &raw_work_days);
            return Err(Box::new(e));
        }
    };

    if work_days > 31 {
        error!("31日以上入力しないでください。");
        Err(InputError::WorkDaysOverOneMonthError(work_days))?;
    }

    Ok(work_days)
}
