use std::fmt;

use crate::work_hour::WorkHour;

pub struct Plan {
  number: &'static str,
  percent: f64,
  total_hour: WorkHour,
  work_hours_per_day: f64,
  remark: &'static str,
}

impl Plan {
  pub fn new(number: &'static str, percent: f64, sum_hour: WorkHour, work_days: u8, remark: &'static str) -> Self {
    Plan {
      number: number,
      percent: percent,
      total_hour: sum_hour * percent,
      work_hours_per_day: (sum_hour / work_days as f64).hour(),
      remark: remark,
    }
  }

  pub fn total_hour(&self) -> WorkHour {
    self.total_hour
  }
}

impl fmt::Display for Plan {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
     write!(f, "{num} ({per}%): 累計 {total:2.2} 時間, {day:2.0} 日間と{hour:2.2} 時間 : {remark}",
     num = self.number,
     per = self.percent,
     total = self.total_hour,
     day = (self.total_hour.hour() / self.work_hours_per_day) as i32,
     hour = self.total_hour.hour() % self.work_hours_per_day,
     remark = self.remark
   )
  }
}
