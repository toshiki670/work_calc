

pub fn worktime(title: &str, per: &f32, sum: &f32) {
  // 一日の労働時間
  let man_day: f32 = 7.75;
  println!("{title} ({per}): 累計 {sum:2.2} 時間, {day:2.0} 日間と{hour:2.2} 時間",
            title = title,
            per = per,
            sum = sum,
            day = sum / man_day,
            hour = sum % man_day);
}