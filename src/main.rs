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

  print::worktime("191852-24", &service, &service_sum, "SBNサービス運営_2021年05月/MC運用業務（業託）MC運用業務（業託） ※25日までに入力");
  print::worktime("206174-01", &portal , &portal_sum , "SBN_クラウドポータルv1.24開発");
  print::worktime("206175-01", &gw     , &gw_sum     , "SBN_クラウドGW_v1.24開発");
}
