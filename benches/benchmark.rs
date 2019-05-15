#[macro_use]
extern crate criterion;

use criterion::Criterion;

extern crate agp_lib;

use agp_lib::parser;
use agp_lib::static_scheduling::algorithms::{cpfd, etf, hlfet, random};

fn static_all_schedule_file(filepath: &std::path::PathBuf) {
    let mut graph =
        parser::parse(&filepath.to_str().unwrap()).expect("Failed parsing the audio graph\n");

    let _etf_schedule = etf(&mut graph, 5);

    let _random_schedule = random(&mut graph, 5);
    let _hlfet_schedule = hlfet(&mut graph, 5);

    let _cpfd_schedule_no_com = cpfd(&mut graph, 0.0);

    let _cpfd_schedule = cpfd(&mut graph, 1.0);
}

fn etf_schedule(filepath: &std::path::PathBuf) {
    let mut graph =
        parser::parse(&filepath.to_str().unwrap()).expect("Failed parsing the audio graph\n");

    let _etf_schedule = etf(&mut graph, 5);
}

fn ramdom_schedule(filepath: &std::path::PathBuf) {
    let mut graph =
        parser::parse(&filepath.to_str().unwrap()).expect("Failed parsing the audio graph\n");

    let _random_schedule = random(&mut graph, 5);
}

fn hlfet_schedule(filepath: &std::path::PathBuf) {
    let mut graph =
        parser::parse(&filepath.to_str().unwrap()).expect("Failed parsing the audio graph\n");
    let _hlfet_schedule = hlfet(&mut graph, 5);
}

fn cpfd_schedule(filepath: &std::path::PathBuf) {
    let mut graph =
        parser::parse(&filepath.to_str().unwrap()).expect("Failed parsing the audio graph\n");

    let _cpfd_schedule = cpfd(&mut graph, 1.0);
}
fn little_random_10_benchmark(c: &mut Criterion) {
    let paths = vec![
        "Samples/AG/little_random_graphs/rand-10-node-graph-0-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-1-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-2-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-4-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-6-ex-20.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-16-ex-162.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-0-ex-7.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-8-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-7-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-3-ex-7.ag",
    ];
    for i in 0..10 {
        let path = std::path::PathBuf::from(paths[i]);
        let name = format!("little_random_bench_{}", i);
        c.bench_function(name.as_str(), move |b| {
            b.iter(|| static_all_schedule_file(&path))
        });
    }
}

fn etf_little_random_10_benchmark(c: &mut Criterion) {
    let paths = vec![
        "Samples/AG/little_random_graphs/rand-10-node-graph-0-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-1-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-2-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-4-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-6-ex-20.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-16-ex-162.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-0-ex-7.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-8-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-7-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-3-ex-7.ag",
    ];
    for i in 0..10 {
        let path = std::path::PathBuf::from(paths[i]);
        let name = format!("etf_little_random_bench_{}", i);
        c.bench_function(name.as_str(), move |b| b.iter(|| etf_schedule(&path)));
    }
}

fn hlfet_little_random_10_benchmark(c: &mut Criterion) {
    let paths = vec![
        "Samples/AG/little_random_graphs/rand-10-node-graph-0-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-1-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-2-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-4-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-6-ex-20.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-16-ex-162.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-0-ex-7.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-8-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-7-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-3-ex-7.ag",
    ];
    for i in 0..10 {
        let path = std::path::PathBuf::from(paths[i]);
        let name = format!("hlfet_little_random_bench_{}", i);
        c.bench_function(name.as_str(), move |b| b.iter(|| hlfet_schedule(&path)));
    }
}

fn random_little_random_10_benchmark(c: &mut Criterion) {
    let paths = vec![
        "Samples/AG/little_random_graphs/rand-10-node-graph-0-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-1-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-2-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-4-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-6-ex-20.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-16-ex-162.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-0-ex-7.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-8-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-7-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-3-ex-7.ag",
    ];
    for i in 0..10 {
        let path = std::path::PathBuf::from(paths[i]);
        let name = format!("random_little_random_bench_{}", i);
        c.bench_function(name.as_str(), move |b| b.iter(|| ramdom_schedule(&path)));
    }
}

fn cpfd_little_random_10_benchmark(c: &mut Criterion) {
    let paths = vec![
        "Samples/AG/little_random_graphs/rand-10-node-graph-0-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-1-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-2-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-4-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-6-ex-20.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-16-ex-162.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-0-ex-7.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-8-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-7-ex-1.ag",
        "Samples/AG/little_random_graphs/rand-10-node-graph-3-ex-7.ag",
    ];
    for i in 0..10 {
        let path = std::path::PathBuf::from(paths[i]);
        let name = format!("cpfd_little_random_bench_{}", i);
        c.bench_function(name.as_str(), move |b| b.iter(|| cpfd_schedule(&path)));
    }
}

criterion_group!(
    benches,
    little_random_10_benchmark,
    etf_little_random_10_benchmark,
    hlfet_little_random_10_benchmark,
    random_little_random_10_benchmark,
    cpfd_little_random_10_benchmark
);

criterion_main!(benches);
