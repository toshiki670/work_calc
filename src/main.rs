use proconio::input;
mod print;
mod work_hour;
use crate::work_hour::WorkHour;


fn main() {
  println!("Enter man hour:");
  input! {
    sum_hour: f32,
  }
  let sum_hour = WorkHour::new(sum_hour);

  assert!(140. <= sum_hour.raw(), "一人月の労働時間は140時間以上にしてください。");
  assert!(0. >= sum_hour.reminder(), "{:.2}時間余分です。労働時間は15分刻みで入力してください。", sum_hour.reminder());

  println!("Enter work days:");
  input! {
    work_days: u8,
  }
  assert!(work_days <= 31, "31日以上入力しないでください。");


  // SBN_クラウドポータルv1.24開発
  let portal: f32 = 0.5;
  let portal_sum = sum_hour * portal;

  // SBN_クラウドGW
  let gw: f32 = 0.2;
  let gw_sum = sum_hour * gw;

  // SBNサービス運営
  let service: f32 = 0.3;
  let service_sum = sum_hour * service;

  assert_eq!(sum_hour, portal_sum + gw_sum + service_sum, "計算結果と合計時間が異なる。");

  let printer = print::Printer::new(sum_hour, work_days);

  printer.worktime("191852-24", &service, &service_sum, "SBNサービス運営_2021年05月/MC運用業務（業託）MC運用業務（業託） ※25日までに入力");
  printer.worktime("206174-01", &portal , &portal_sum , "SBN_クラウドポータルv1.24開発");
  printer.worktime("206175-01", &gw     , &gw_sum     , "SBN_クラウドGW_v1.24開発");
}
