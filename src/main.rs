use std::env;

use clap::App;
use env_logger;
use log::Level;

mod input;
mod case;
mod setting;
mod validation;
mod work_hour;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(cli).get_matches();

    if matches.is_present("verbose") {
        env::set_var("RUST_LOG", Level::Trace.to_string());
    }
    env_logger::init();

    let setting = setting::Setting::read(matches.value_of("setting"))?;

    // Total hour
    let total_hour =
        input::get_total_hour(matches.value_of("total_hour"), &setting.general.total_hour)?;

    // Work days
    let work_days =
        input::get_work_days(matches.value_of("work_days"), &setting.general.work_days)?;

    // Instantiate cases
    let mut cases: Vec<case::Case> = Vec::new();
    for p in setting.cases {
        cases.push(case::Case::new(
            p.number, p.percent, total_hour, work_days, p.remark,
        ));
    }

    // Validations
    validation::valid_total_percent(&cases)?;
    validation::valid_equals_total_hour_and_cases_total_hour(&total_hour, &cases)?;

    println!(
        "一日の基本労働時間: {:.2} 時間",
        (total_hour / work_days as f64).hour()
    );

    for case in cases.iter() {
        println!("{}", case);
    }
    println!();

    let case_work_days = cases.iter().fold(0, |sum, p| sum + p.work_days());
    println!(
        "各案件で分割不可能な日数 (各案件の余り時間を入力): {:1.0} 日",
        work_days - case_work_days
    );

    let case_total_hour = cases.iter().fold(0.0, |sum, p| sum + p.total_hour().hour());
    println!(
        "各案件で分割不可能な時間 (各案件で分割不可能な日数に追加): {:.2} 時間",
        total_hour - case_total_hour
    );

    Ok(())
}
