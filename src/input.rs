use log::{debug, error};
use std::error::Error;
use std::{fmt, io};

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
    cli_total_hour: &Option<String>,
    setting: &Option<String>,
) -> Result<WorkHour, Box<dyn Error>> {
    let mut stdin_line = String::new();

    // Total hour
    let raw_total_hour = match cli_total_hour {
        Some(hour) => hour,
        None => match setting {
            Some(hour) => hour,
            None => {
                read_stdin_line("合計時間", &mut stdin_line)?;
                stdin_line.trim()
            }
        },
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
    cli_work_days: &Option<String>,
    setting: &Option<String>,
) -> Result<u8, Box<dyn Error>> {
    let mut stdin_line = String::new();

    // Work days
    let raw_work_days = match cli_work_days {
        Some(hour) => hour,
        None => match setting {
            Some(hour) => hour,
            None => {
                read_stdin_line("労働日数", &mut stdin_line)?;
                stdin_line.trim()
            }
        },
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

fn read_stdin_line(title: &str, stdin_line: &mut String) -> Result<(), std::io::Error> {
    println!("{} を入力してください：", title);

    if let Err(e) = io::stdin().read_line(stdin_line) {
        error!("標準入力に失敗しました。");
        return Err(e);
    }
    println!();

    Ok(())
}
