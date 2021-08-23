use std::env;

use advent_of_code::utils::{read, GenericResult};
use anyhow::Context;
use itertools::Itertools;
use paste::paste;

fn main() -> GenericResult<()> {
    let args = env::args().collect_vec();
    if args.len() != 3 {
        panic!("Invalid arguments.");
    }

    let year = args[1]
        .parse::<u16>()
        .with_context(|| format!("Cannot parse year \"{}\".", args[1]))?;
    let day = args[2]
        .parse::<u16>()
        .with_context(|| format!("Cannot parse day \"{}\".", args[2]))?;

    macro_rules! gen_match {
        ($year:expr; $($n:expr)+ ) => {
            paste!{
                use advent_of_code::[<y $year>]::*;

                #[allow(clippy::zero_prefixed_literal)]
                $(
                    if day == $n {
                        let data = read($year, concat!(stringify!([<day$n>]), ".txt"));
                        println!("{:?}", [<day$n>]::combi(&[<day$n>]::parse(&data)?));
                        return Ok(());
                    }
                )+
                panic!(concat!("Invalid day for ", stringify!($year), "."));
            };
        }
    }

    if year == 2018 {
        gen_match! {2018; 01};
    } else if year == 2019 {
        panic!("Not implemented");
    }

    panic!("Invalid year");
}
