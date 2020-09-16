const TIME_PER_COMPUTATION: u64 = 1;

use ralgorithms::perf;

fn main() {
    for time_period in vec![
        1_000_000,
        1_000_000 * 60,
        1_000_000 * 60 * 60,
        1_000_000 * 60 * 60 * 24,
        1_000_000 * 60 * 60 * 24 * 30,
        1_000_000 * 60 * 60 * 24 * 365,
        1_000_000 * 60 * 60 * 24 * 365 * 100,
    ] {
        println!("Running for {:e} nanoseconds", &time_period);
        for algo in vec![
            // perf::Complexity::LOG, // Overflows u128
            perf::Complexity::SQRT,
            perf::Complexity::LINEAR,
            perf::Complexity::NLOGN,
            perf::Complexity::QUADRATIC,
            perf::Complexity::CUBIC,
            perf::Complexity::EXPONENTIAL,
            perf::Complexity::FACTORIAL,
        ] {
            let computations = perf::executions_per(&algo, &time_period, &TIME_PER_COMPUTATION);
            println!(
                "For algorithm {:?}, can do {:e} computations",
                algo, computations
            );
        }
        println!("");
    }
}
