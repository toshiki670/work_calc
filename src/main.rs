use proconio::input;

fn print_worktime(title: &str, per: &f32, sum: &f32) {
  // 一日の労働時間
  let man_day: f32 = 7.75;
  println!("{title} ({per}): 累計 {sum:2.2} 時間, {day:2.0} 日間と{hour:2.2} 時間",
            title = title,
            per = per,
            sum = sum,
            day = sum / man_day,
            hour = sum % man_day);
}

fn main() {
  input! {
    sum_hour: f32,
  }

  // SBN_クラウドポータルv1.24開発
  let portal: f32 = 0.5;
  let portal_sum: f32 = sum_hour * portal;

  // SBN_クラウドGW
  let gw: f32 = 0.2;
  let gw_sum: f32 = sum_hour * gw;

  // SBNサービス運営
  let service: f32 = 0.3;
  let service_sum: f32 = sum_hour * service;

  assert_eq!(sum_hour, portal_sum + gw_sum + service_sum, "計算結果と合計時間が異なる。");

  print_worktime("SBN_クラウドポータルv1.24開発", &portal, &portal_sum);
  print_worktime("SBN_クラウドGW_v1.24開発     ", &gw, &gw_sum);
  print_worktime("SBNサービス運営              ", &service, &service_sum);
}
