use log::error;
use std::error::Error;
use std::fmt;

use crate::plan;

use crate::work_hour::WorkHour;

#[derive(Debug)]
pub enum ValidError {
    NotEqualsTotalHourAndPlansTotalHourError,
}
impl Error for ValidError {}

impl fmt::Display for ValidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidError::NotEqualsTotalHourAndPlansTotalHourError => {
                write!(f, "NotEqualsTotalHourAndPlansTotalHourError")
            }
        }
    }
}

pub fn valid_equals_total_hour_and_plans_total_hour(
    total_hour: &WorkHour,
    plans: &Vec<plan::Plan>,
) -> Result<(), Box<dyn Error>> {
    let plan_total_hour = plans
        .iter()
        .fold(WorkHour::new(0.0), |sum, p| sum + p.total_hour());
    if *total_hour == plan_total_hour {
        Ok(())
    } else {
        error!("計算結果と合計時間が不一致: {} != {}", total_hour, plan_total_hour);
        Err(Box::new(
            ValidError::NotEqualsTotalHourAndPlansTotalHourError,
        ))
    }
}
