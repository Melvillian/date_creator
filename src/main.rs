use structopt::StructOpt;
use structopt::clap::arg_enum;

#[derive(Debug, StructOpt)]
#[structopt(name = "date generator", about = "small CLI utility to generate dates for my google docs")]
struct Opt {
    #[structopt(possible_values = &Month::variants(), case_insensitive = true)]
    month: Month,

    start: u8,

    end: u8,
}

arg_enum! {
    #[derive(Debug)]
    enum Month {
        January,
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

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
