use std::{ascii::AsciiExt, process::exit, vec};

use chrono::Datelike;
use clap::{ArgEnum, Parser};
use select::{document::Document, predicate::Attr};
use termion::{color, style};
use dict:: {Dict, DictIface};
const WEEK_DAYS: [&str; 5] = [
    "Segunda-Feira",
    "Terça-Feira",
    "Quarta-Feira",
    "Quinta-Feira",
    "Sexta-Feira",
];

#[macro_export]
macro_rules! bold {
    ($text:expr) => {
        format!("{}{}{}", style::Bold, $text, style::Reset)
    };
}

#[macro_export]
macro_rules! underline {
    ($text:expr) => {
        format!("{}{}{}", style::Underline, $text, style::Reset)
    };
}

#[macro_export]
macro_rules! red {
    ($text:expr) => {
        format!("{}{}{}", color::Fg(color::Red), $text, style::Reset)
    };
}

fn ementa(day: usize, all: bool) {
    let url = "https://eatdreamsmile.pt/";
    let mut current_day = String::new();
    let mut info = Vec::<String>::new();
    for day in WEEK_DAYS {
        current_day = url.to_string();
        current_day.push_str(day.to_lowercase().replace("ç", "c").as_str());

        let response = reqwest::blocking::get(current_day).unwrap().text().unwrap();
        let document = Document::from(response.as_str());

        println!("{}", day);
        let mut i = 0;
        let mut z = 0;
        let mut lunch = true;
        let mut dic = Dict::<String>::new();
        for node in document.find(Attr("class", "wpb_wrapper")) {
            if i>=3 {
                for child in node.children() {
                    if child.name() == Some("h4") {
                        
                        info.push(child.text());

                        z += 1;
                    }
                }
                
            }
            i += 1;
        }
        println!("{:?}", info);
        break;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum WeekdayArg {
    Seg,
    Ter,
    Qua,
    Qui,
    Sex,
}

/// Command line tool to fetch the menu of the IST Alameda canteen
///
/// When no argument is provided, today's menu is shown
#[derive(Parser, Debug)]
struct Args {
    /// Prints all the menus of the week
    #[clap(short, long)]
    all: bool,

    /// Prints the menu from that day
    #[clap(short, long, arg_enum)]
    day: Option<WeekdayArg>,
}

fn main() {
    let args = Args::parse();

    // Convert the day argument from week day to an integer starting in monday with 0
    let day = match args.day {
        Some(WeekdayArg::Seg) => 0,
        Some(WeekdayArg::Ter) => 1,
        Some(WeekdayArg::Qua) => 2,
        Some(WeekdayArg::Qui) => 3,
        Some(WeekdayArg::Sex) => 4,
        None => chrono::offset::Local::today()
            .weekday()
            .num_days_from_monday()
            .try_into()
            .unwrap(),
    };

    ementa(day, args.all);
}
