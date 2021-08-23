use proconio::input;

mod plan;
mod work_hour;
use crate::work_hour::WorkHour;


fn main() {
  println!("Enter total hour:");
  input! {
    sum_hour: f64,
  }
  let sum_hour = WorkHour::new(sum_hour);

  assert!(140. <= sum_hour.raw(), "一人月の労働時間は140時間以上にしてください。");
  assert!(0. >= sum_hour.reminder(), "{:.2}時間余分です。労働時間は15分刻みで入力してください。", sum_hour.reminder());

  println!("Enter work days:");
  input! {
    work_days: u8,
  }
  assert!(work_days <= 31, "31日以上入力しないでください。");


  // SBN_クラウドポータルv1.25開発
  let portal_plan = plan::Plan::new(
    "210915-01",
    0.5,
    sum_hour,
    work_days,
    "SBN_クラウドポータルv1.25開発"
  );

  // SBN_クラウドGW
  let gw_cloud_plan = plan::Plan::new(
    "206175-01",
    0.2,
    sum_hour,
    work_days,
    "SBN_クラウドGW_v1.24開発"
  );


  // SBNサービス運営
  let service_plan = plan::Plan::new(
    "191852-27",
    0.3,
    sum_hour,
    work_days,
    "SBNサービス運営_2021年08月 / MC運用業務（業託）MC運用業務（業託） ※25日までに入力すること"
  );


  let plan_sum = portal_plan.total_hour() + gw_cloud_plan.total_hour() + service_plan.total_hour();
  assert_eq!(sum_hour, plan_sum, "計算結果と合計時間が異なる。");


  println!("一日の基本労働時間: {:.2} 時間", (sum_hour / work_days as f64).hour());
  println!("{}", service_plan);
  println!("{}", gw_cloud_plan);
  println!("{}", portal_plan);

  println!();

  let plan_work_days = portal_plan.work_days() + gw_cloud_plan.work_days() + service_plan.work_days();
  println!("プロジェクト間分割不可能日数 (各プロジェクトの余り時間を入力): {:1.0} 日", work_days - plan_work_days);

  let plan_sum_hour = portal_plan.total_hour().hour() + gw_cloud_plan.total_hour().hour() + service_plan.total_hour().hour();
  println!("プロジェクト間分割不可能時間 (プロジェクト間分割不可能日に追加): {:.2} 時間", sum_hour - plan_sum_hour);
}
