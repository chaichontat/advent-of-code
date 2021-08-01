use std::env;

use advent_of_code::utils::read;
use itertools::Itertools;
use paste::paste;

fn main() {
    let args = env::args().collect_vec();
    if args.len() != 3 {
        panic!("Invalid arguments.");
    }

    let year = args[1].parse::<u16>().unwrap();
    let day = args[2].parse::<u16>().unwrap();

    macro_rules! gen_match {
        ($year:expr; $($n:expr)+ ) => {
            paste!{
                use advent_of_code::[<y $year>]::*;

                #[allow(clippy::zero_prefixed_literal)]
                let res = match day {
                    $(
                        $n => [<day$n>]::combi(&[<day$n>]::parse(&read($year, concat!(stringify!([<day$n>]), ".txt")))).unwrap(),
                    )+
                    _ => panic!(concat!("Invalid day for ", stringify!($year), ".")),
                };
                println!("{:?}", res);
            }
        };
    }

    if year == 2018 {
        gen_match! {2018; 01 18};
    } else if year == 2019 {
    } else {
        panic!("Invalid year");
    }
}
