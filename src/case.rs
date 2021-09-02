use std::fmt;

use crate::work_hour::WorkHour;

#[derive(Debug)]
pub struct Case {
    number: String,
    percent: f64,
    total_hour: WorkHour,
    work_hours_per_day: f64,
    remark: String,
}

impl Case {
    pub fn new(
        number: String,
        percent: f64,
        total_hour: WorkHour,
        work_days: u8,
        remark: String,
    ) -> Self {
        let work_hours_per_day = (total_hour / work_days as f64).hour();

        Case {
            number: number,
            percent: percent,
            total_hour: total_hour * percent,
            work_hours_per_day: work_hours_per_day,
            remark: remark,
        }
    }

    pub fn total_hour(&self) -> WorkHour {
        self.total_hour
    }

    pub fn work_days(&self) -> u8 {
        (self.total_hour.hour() / self.work_hours_per_day) as u8
    }

    pub fn percent(&self) -> f64 {
        self.percent
    }
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 分割できなかった余り時間
        let rem_hour = self.total_hour.hour() % self.work_hours_per_day;

        write!(f, "{num} ({per}%): 累計 {total:2.2} 時間, {day:2.0} 日間, 余り {rem_hour:2.2} 時間 : {remark}",
            num = self.number,
            per = self.percent,
            total = self.total_hour,
            day = self.work_days(),
            rem_hour = rem_hour,
            remark = self.remark
        )
    }
}
