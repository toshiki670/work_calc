use proconio::input;
mod print;


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

  print::worktime("191852-24 SBNサービス運営 MC運用業務（業託）", &service, &service_sum);
  print::worktime("206174-01 SBN_クラウドポータルv1.24開発     ", &portal, &portal_sum);
  print::worktime("206175-01 SBN_クラウドGW_v1.24開発          ", &gw, &gw_sum);
}
