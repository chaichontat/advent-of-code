#[macro_use]
extern crate criterion;
extern crate advent_of_code;
use advent_of_code::utils;
use criterion::{black_box, Criterion};
use pprof::criterion::{Output, PProfProfiler};

fn bench_2019(c: &mut Criterion) {
    use advent_of_code::y2019::*;
    let read = |p| utils::read(2019, p);

    let day01 = &day01::parse(&read("day01.txt"));
    c.bench_function("day01a", |b| b.iter(|| day01::part1(black_box(day01))));
    c.bench_function("day01b", |b| b.iter(|| day01::part2(black_box(day01))));

    let day02 = &day02::parse(&read("day02.txt"));
    c.bench_function("day02combi", |b| b.iter(|| day02::run(black_box(day02))));

    let day03 = &day03::parse(&read("day03.txt"));
    c.bench_function("day03a", |b| b.iter(|| day03::part1(black_box(day03))));
    c.bench_function("day03b", |b| b.iter(|| day03::part2(black_box(day03))));

    let day04 = &day04::parse(&read("day04.txt"));
    c.bench_function("day04a", |b| b.iter(|| day04::part1(black_box(day04))));
    c.bench_function("day04b", |b| b.iter(|| day04::part2(black_box(day04))));

    let day05 = &day05::parse(&read("day05.txt"));
    c.bench_function("day05a", |b| b.iter(|| day05::part1(black_box(day05))));
    c.bench_function("day05b", |b| b.iter(|| day05::part2(black_box(day05))));

    let day06 = &day06::parse(&read("day06.txt"));
    c.bench_function("day06a", |b| b.iter(|| day06::part1(black_box(day06))));
    c.bench_function("day06b", |b| b.iter(|| day06::part2(black_box(day06))));

    let day07 = &day07::parse(&read("day07.txt"));
    c.bench_function("day07a", |b| b.iter(|| day07::part1(black_box(day07))));
    c.bench_function("day07b", |b| b.iter(|| day07::part2(black_box(day07))));

    let day08 = &day08::parse(&read("day08.txt"));
    c.bench_function("day08a", |b| b.iter(|| day08::part1(black_box(day08))));
    c.bench_function("day08b", |b| b.iter(|| day08::part2(black_box(day08))));

    let day09 = &day09::parse(&read("day09.txt"));
    c.bench_function("day09a", |b| b.iter(|| day09::part1(black_box(day09))));
    c.bench_function("day09b", |b| b.iter(|| day09::part2(black_box(day09))));

    let day10 = &day10::parse(&read("day10.txt"));
    c.bench_function("day10a", |b| b.iter(|| day10::bench(black_box(day10))));

    let day11 = &day11::parse(&read("day11.txt"));
    c.bench_function("day11a", |b| b.iter(|| day11::part1(black_box(day11))));
    c.bench_function("day11b", |b| b.iter(|| day11::part2(black_box(day11))));

    let day12 = &day12::parse(&read("day12.txt"));
    c.bench_function("day12a", |b| b.iter(|| day12::part1(black_box(day12))));
    c.bench_function("day12b", |b| b.iter(|| day12::part2(black_box(day12))));

    let day13 = &day13::parse(&read("day13.txt"));
    c.bench_function("day13a", |b| b.iter(|| day13::part1(black_box(day13))));
    c.bench_function("day13b", |b| b.iter(|| day13::part2(black_box(day13))));

    let day14 = &day14::parse(&read("day14.txt"));
    c.bench_function("day14a", |b| b.iter(|| day14::part1(black_box(day14))));
    c.bench_function("day14b", |b| b.iter(|| day14::part2(black_box(day14))));

    let day15 = &day15::parse(&read("day15.txt"));
    c.bench_function("day15a", |b| b.iter(|| day15::part1(black_box(day15))));
    c.bench_function("day15b", |b| b.iter(|| day15::part2(black_box(day15))));

    let day16 = &day16::parse(&read("day16.txt"));
    c.bench_function("day16a", |b| b.iter(|| day16::part1(black_box(day16))));
    c.bench_function("day16b", |b| b.iter(|| day16::part2(black_box(day16))));

    let day17 = &day17::parse(&read("day17.txt"));
    c.bench_function("day17a", |b| b.iter(|| day17::part1(black_box(day17))));
    c.bench_function("day17b", |b| b.iter(|| day17::part2(black_box(day17))));

    let day18 = &day18::parse(&read("day18.txt"));
    let mut day18b = day18::parse(&read("day18.txt"));
    let xmid = (day18b[0].len() - 1) / 2;
    let ymid = (day18b.len() - 1) / 2;
    day18b[ymid - 1].replace_range(xmid - 1..=xmid + 1, "@#@");
    day18b[ymid].replace_range(xmid - 1..=xmid + 1, "###");
    day18b[ymid + 1].replace_range(xmid - 1..=xmid + 1, "@#@");

    c.bench_function("day18a", |b| b.iter(|| day18::run(black_box(day18))));
    c.bench_function("day18b", |b| b.iter(|| day18::run(black_box(&day18b))));

    let day19 = &day19::parse(&read("day19.txt"));
    c.bench_function("day19a", |b| b.iter(|| day19::part1(black_box(day19))));
    c.bench_function("day19b", |b| b.iter(|| day19::part2(black_box(day19))));

    let day20 = &day20::parse(&read("day20.txt"));
    c.bench_function("day20a", |b| b.iter(|| day20::part1(black_box(day20))));
    c.bench_function("day20b", |b| b.iter(|| day20::part2(black_box(day20))));

    let day21 = &day21::parse(&read("day21.txt"));
    c.bench_function("day21a", |b| b.iter(|| day21::part1(black_box(day21))));
    c.bench_function("day21b", |b| b.iter(|| day21::part2(black_box(day21))));

    let day22 = &day22::parse(&read("day22.txt"));
    c.bench_function("day22a", |b| b.iter(|| day22::part1(black_box(day22))));
    c.bench_function("day22b", |b| b.iter(|| day22::part2(black_box(day22))));

    let day23 = &day23::parse(&read("day23.txt"));
    c.bench_function("day23a", |b| b.iter(|| day23::part1(black_box(day23))));
    c.bench_function("day23b", |b| b.iter(|| day23::part2(black_box(day23))));

    let day24 = &day24::parse(&read("day24.txt"));
    c.bench_function("day24a", |b| b.iter(|| day24::part1(black_box(day24))));
    c.bench_function("day24b", |b| b.iter(|| day24::part2(black_box(day24))));

    let day25 = &day25::parse(&read("day25.txt"));
    c.bench_function("day25a", |b| b.iter(|| day25::part1(black_box(day25))));
}

fn bench_2018(c: &mut Criterion) {
    use advent_of_code::y2018::*;
    let read = |p| utils::read(2018, p);

    let day01 = &day01::parse(&read("day01.txt"));
    c.bench_function("day01c", |b| b.iter(|| day01::combi(black_box(day01))));

    let day02 = &day02::parse(&read("day02.txt"));
    c.bench_function("day02a", |b| b.iter(|| day02::part1(black_box(day02))));
    c.bench_function("day02b", |b| b.iter(|| day02::part2_simd(black_box(day02))));

    let day07 = &day07::parse(&read("day07.txt"));
    c.bench_function("2018_day07", |b| b.iter(|| day07::combi(black_box(day07))));
    let day11 = &day11::parse(&read("day11.txt"));
    c.bench_function("day11c", |b| b.iter(|| day11::combi(black_box(day11))));

    let day13 = &day13::parse(&read("day13.txt"));
    c.bench_function("2018_day13", |b| b.iter(|| day13::combi(black_box(day13))));

    let day15 = &day15::parse(&read("day15.txt"));
    c.bench_function("2018_day15", |b| b.iter(|| day15::combi(black_box(day15))));

    let day16 = &day16::parse(&read("day16.txt"));
    c.bench_function("day16c", |b| b.iter(|| day16::combi(black_box(day16))));
    unsafe {
        c.bench_function("day16c_unchecked", |b| {
            b.iter(|| day16::combi_unchecked(black_box(day16)))
        });
    }

    let day17 = &day17::parse(&read("day17.txt"));
    c.bench_function("2018_day17", |b| b.iter(|| day17::combi(black_box(day17))));
    let day18 = &day18::parse(&read("day18.txt"));
    c.bench_function("2018_day18", |b| b.iter(|| day18::combi(black_box(day18))));

    let day19 = &day19::parse(&read("day19.txt"));
    c.bench_function("day19c", |b| b.iter(|| day19::combi(black_box(day19))));
    let day20 = &day20::parse(&read("day20.txt"));
    c.bench_function("2018_day20", |b| b.iter(|| day20::combi(black_box(day20))));
    let day22 = &day22::parse(&read("day22.txt"));
    c.bench_function("2018_day22", |b| b.iter(|| day22::combi(black_box(day22))));

    let day24 = &day24::parse(&read("day24.txt"));
    c.bench_function("2018_day24", |b| b.iter(|| day24::combi(black_box(day24))));
    let day25 = &day25::parse(&read("day25.txt"));
    c.bench_function("2018_day25", |b| b.iter(|| day25::part1(black_box(day25))));
}

fn bench_2017(c: &mut Criterion) {
    use advent_of_code::y2017::*;
    let read = |p| utils::read(2018, p);

    let day01 = &day01::parse(&read("day01.txt"));
    let day01 = &day01::parse("181445682966897848665963472661939865313976877194312684993521259486517527961396717561854825453963181134379574918373213732184697746668399631642622373684425326112585283946462323363991753895647177797691214784149215198715986947573668987188746878678399624533792551651335979847131975965677957755571358934665327487287312467771187981424785514785421781781976477326712674311994735947987383516699897916595433228294198759715959469578766739518475118771755787196238772345762941477359483456641194685333528329581113788599843621326313592354167846466415943566183192946217689936174884493199368681514958669615226362538622898367728662941275658917124167353496334664239539753835439929664552886538885727235662548783529353611441231681613535447417941911479391558481443933134283852879511395429489152435996669232681215627723723565872291296878528334773391626672491878762288953597499218397146685679387438634857358552943964839321464529237533868734473777756775687759355878519113426969197211824325893376812556798483325994128743242544899625215765851923959798197562831313891371735973761384464685316273343541852758525318144681364492173465174512856618292785483181956548813344752352933634979165667651165776587656468598791994573513652324764687515345959621493346623821965554755615219855842969932269414839446887613738174567989512857785566352285988991946436148652839391593178736624957214917527759574235133666461988355855613377789115472297915429318142824465141688559333787512328799783539285826471818279818457674417354335454395644435889386297695625378256613558911695145397779576526397241795181294322797687168326696497256684943829666672341162656479563522892141714998477865114944671225898297338685958644728534192317628618817551492975251364233974374724968483637518876583946828819994321129556511537619253381981544394112184655586964655164192552352534626295996968762388827294873362719636616182786976922445125551927969267591395292198155775434997827738862786341543524544822321112131815475829945625787561369956264826651461575948462782869972654343749617939132353399334744265286151177931594514857563664329299713436914721119746932159456287267887878779218815883191236858656959258484139254446341");
    c.bench_function("2017_day01", |b| b.iter(|| day01::combi(black_box(day01))));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Protobuf));
    targets = bench_2019, bench_2018, bench_2017
}

criterion_main!(benches);
