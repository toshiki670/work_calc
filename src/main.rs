use std::env;

use clap::App;
use env_logger;
use log::{debug, error, Level};

mod input;
mod plan;
mod setting;
mod work_hour;
use crate::work_hour::WorkHour;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(cli).get_matches();

    if matches.is_present("verbose") {
        env::set_var("RUST_LOG", Level::Trace.to_string());
    }
    env_logger::init();

    let setting = setting::Setting::read(matches.value_of("setting"))?;

    // Total hour
    let raw_total_hour = matches.value_of("total_hour").unwrap();
    let total_hour = raw_total_hour.parse();
    if let Err(e) = total_hour {
        error!("合計時間が誤っています。");
        error!("数字であることを確認してください: {}.", &raw_total_hour);
        panic!("ParseError: {}.", e);
    }

    let total_hour = WorkHour::new(total_hour.unwrap());
    debug!("total_hour({}): {:?}", &total_hour, &total_hour);
    if 140. > total_hour.raw() {
        error!("一人月の労働時間は140時間以上にしてください。");
        panic!("Over 140 hour.");
    } else if 0. < total_hour.reminder() {
        error!(
            "{:.2}時間余分です。労働時間は15分刻みで入力してください。",
            total_hour.reminder()
        );
        panic!("Error {}.", total_hour.reminder());
    }

    // Work days
    let raw_work_days = matches.value_of("work_days").unwrap();
    let work_days = raw_work_days.parse();
    if let Err(e) = work_days {
        error!("労働日数が誤っています。");
        error!("数字であることを確認してください: {}.", &raw_work_days);
        panic!("ParseError: {}.", e);
    }
    let work_days = work_days.unwrap();

    if work_days > 31 {
        error!("31日以上入力しないでください。");
        panic!("work_days({}) exceeded 31.", work_days);
    }

    let mut plans: Vec<plan::Plan> = Vec::new();
    for p in setting.plans {
        plans.push(plan::Plan::new(
            p.number, p.percent, total_hour, work_days, p.remark,
        ));
    }

    let plan_sum = plans
        .iter()
        .fold(WorkHour::new(0.0), |sum, p| sum + p.total_hour());
    if total_hour != plan_sum {
        error!("計算結果と合計時間が異なる。");
        panic!(
            "total_hour({}) and plan_sum({}) are different",
            total_hour, plan_sum
        );
    }

    println!(
        "一日の基本労働時間: {:.2} 時間",
        (total_hour / work_days as f64).hour()
    );

    for plan in plans.iter() {
        println!("{}", plan);
    }
    println!();

    let plan_work_days = plans.iter().fold(0, |sum, p| sum + p.work_days());
    println!(
        "プロジェクト間分割不可能日数 (各プロジェクトの余り時間を入力): {:1.0} 日",
        work_days - plan_work_days
    );

    let plan_total_hour = plans.iter().fold(0.0, |sum, p| sum + p.total_hour().hour());
    println!(
        "プロジェクト間分割不可能時間 (プロジェクト間分割不可能日に追加): {:.2} 時間",
        total_hour - plan_total_hour
    );

    Ok(())
}
