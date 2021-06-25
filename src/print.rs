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
}