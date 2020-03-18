//! A small CLI utility to generate dates for my google docs Life Journal

use chrono::offset::TimeZone;
use chrono::Datelike;
use chrono::{Date, Utc, Weekday};
use structopt::clap::arg_enum;
use structopt::StructOpt;
use time::Duration as Time_Duration;

const YEAR: i32 = 2020;
const BEGINNING_OF_LAST_WEEK: u64 = (60 * 60 * 24 * 6);

#[derive(Debug, StructOpt)]
#[structopt(
    name = "date generator",
    about = "small CLI utility to generate dates for my google docs"
)]
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
            otherwise => panic!("bad number for Month: {}", otherwise),
        }
    }
}

fn main() {
    let opt = Opt::from_args();

    print_dates(opt);
}

/// Prints date separators for Alex's Google Docs Life Journal
/// # Examples
/// ```
/// // $> cargo run March 3 4
/// // calls
/// print_dates(opt);
/// // which prints:
/// // ** Wed Mar 4 **
/// //
/// //
/// // ** Tue Mar 3 **
/// ```
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

/// Prints the Recap line every Sunday
fn print_recap_line(dt: Date<Utc>) {
    // figure out which week range this recap should cover
    let one_week = Time_Duration::from_std(std::time::Duration::new(BEGINNING_OF_LAST_WEEK, 0)).unwrap();
    let one_week_ago = dt.checked_sub_signed(one_week).unwrap();
    let month_one_week_ago = Month::from_num(one_week_ago.month());

    // Example: Mar 5
    let begin = format!("{} {}", month_one_week_ago, one_week_ago.day());

    let this_month = Month::from_num(dt.month());

    // Example: Mar 12
    let end = format!("{} {}", this_month, dt.day());

    // Example: Mar 5 - Mar 12
    let range = format!("{} - {}", begin, end);
    println!(
        "******************** Recap: {} **************************\n\n",
        range
    );
}
