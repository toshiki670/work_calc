use crate::WorkHour;

pub struct Printer {
  work_hours_per_day: f32,

}


impl Printer {
  pub fn new(sum_hour: WorkHour, work_days: u8) -> Printer {
    let hw = sum_hour / work_days as f32;
    println!("一日の基本労働時間: {:.2} 時間", hw.hour());

    Printer {
      work_hours_per_day: hw.hour(),
    }
  }

  pub fn worktime(&self, plan_no: &str, per: &f32, sum: &WorkHour, remark: &str) {

    println!("{plan_no} ({per}%): 累計 {sum:2.2} 時間, {day:2.0} 日間と{hour:2.2} 時間 : {remark}",
              plan_no = plan_no,
              per = per,
              sum = sum,
              day = (sum.hour() / self.work_hours_per_day) as i32,
              hour = sum.hour() % self.work_hours_per_day,
              remark = remark
            );
  }

  fn calc_work_hours_per_day(sum_hour: f32, work_days: u8) -> f32 {
    let integer_time: u32 = sum_hour as u32 / work_days as u32;
    let surplus_time: f32 = sum_hour as f32 % work_days as f32;
    let quarter_hour: f32 = 1. / 0.25;
    let add_time: f32 = ((surplus_time * quarter_hour) as u32 / work_days as u32) as f32 / quarter_hour;

    integer_time as f32 + add_time
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn be_valid_calc_work_hours_per_day() {
    assert_eq!(super::Printer::calc_work_hours_per_day(140., 16), 8.75);
  }
}