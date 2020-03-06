use structopt::StructOpt;
use structopt::clap::arg_enum;
use chrono::{Utc, Weekday, Date};
use chrono::offset::TimeZone;
use chrono::Datelike;
use time::Duration as Time_Duration;
use std::time::Duration;

const YEAR: i32 = 2020;
const ONE_WEEK_IN_SECS: u64 = 60 * 60 * 24 * 7;

#[derive(Debug, StructOpt)]
#[structopt(name = "date generator", about = "small CLI utility to generate dates for my google docs")]
struct Opt {
    #[structopt(possible_values = &Month::variants(), case_insensitive = true)]
    month: Month,

    start: u32,

    end: u32,
}

arg_enum! {
    #[derive(Debug, Copy, Clone)]
    enum Month {
        January = 1,
        February,
        March,
        April,
        May,
        June,
        July,
        August,
        September,
        October,
        November,
        December,
    }
}

impl Month {
    fn from_num(num: u32) -> Self {
        match num {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            otherwise => panic!("bad number for Month: {}", otherwise)
        }
    }
}

fn main() {
    let opt = Opt::from_args();

    print_dates(opt);
}

/// Prints date separators for Alex's Google Docs Life Journal
/// # Examples
///
/// print_dates(opt);
/// // prints
/// // ** Wed Mar 4 **
/// //
/// //
/// // ** Tue Mar 3 **
fn print_dates(opt: Opt) {
    let month_num = opt.month as u32;

    for i in (opt.start..opt.end + 1).rev() {

        let dt = Utc.ymd(YEAR, month_num, i);

        if dt.weekday() == Weekday::Sun {
            // Sundays are special because I want to use Sunday to revisit the week's events
            // and write a recap of that week.
            print_recap_line(dt);
        }
        println!("** {:?} {:?} {} **\n\n", dt.weekday(), opt.month, i);
    }
}

fn print_recap_line(dt: Date<Utc>) {
    // figure out which week range this recap should cover
    let one_week = Time_Duration::from_std(Duration::new(ONE_WEEK_IN_SECS, 0)).unwrap();
    let one_week_ago = dt.checked_sub_signed(one_week).unwrap();
    let month_one_week_ago = Month::from_num(one_week_ago.month());

    // Example: Mar 5
    let begin = format!("{} {}", month_one_week_ago, one_week_ago.day());

    let this_month = Month::from_num(dt.month());

    // Example: Mar 12
    let end = format!("{} {}", this_month, dt.day());

    let range = format!("{} - {}",  begin, end);
    println!("******************** Recap: {} **************************\n\n", range);
}