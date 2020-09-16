use std::convert::TryInto;

#[derive(std::fmt::Debug)]
pub enum Complexity {
    LOG,
    SQRT,
    LINEAR,
    NLOGN,
    QUADRATIC,
    CUBIC,
    EXPONENTIAL,
    FACTORIAL,
}

pub fn executions_per(
    complexity: &Complexity,
    time_ns: &u64,
    time_per_computation_ns: &u64,
) -> u128 {
    let time_periods: u64 = time_ns / time_per_computation_ns;
    executions_per_impl(complexity, time_periods)
}

fn executions_per_impl(complexity: &Complexity, time_periods: u64) -> u128 {
    match complexity {
        Complexity::LOG => 2u128.pow(time_periods.try_into().unwrap()),
        Complexity::SQRT => (time_periods as u128 * time_periods as u128),
        Complexity::LINEAR => time_periods as u128,
        Complexity::NLOGN => executions_per_nlogn(time_periods),
        Complexity::QUADRATIC => (time_periods as f64).sqrt() as u128,
        Complexity::CUBIC => (time_periods as f64).cbrt() as u128,
        Complexity::EXPONENTIAL => (time_periods as f64).log(2.0) as u128,
        Complexity::FACTORIAL => executions_per_factorial(time_periods),
    }
}

fn executions_per_nlogn(time_periods: u64) -> u128 {
    use std::cmp::Ordering;
    let mut left = executions_per_impl(&Complexity::QUADRATIC, time_periods) as u64;
    let mut right = executions_per_impl(&Complexity::LINEAR, time_periods) as u64;
    while left < right {
        let mid: u64 = ((left + right) / 2) as u64;
        let time_used = (mid as f64 * (mid as f64).log(2.0)) as u64;
        match time_used.cmp(&time_periods) {
            Ordering::Less => {
                left = mid + 1;
            }
            Ordering::Greater => {
                right = mid;
            }
            Ordering::Equal => break,
        };
    }
    left as u128
}

fn executions_per_factorial(time_periods: u64) -> u128 {
    let mut count: u64 = 1;
    let mut fac: u64 = 1;
    while fac < time_periods {
        count += 1;
        fac *= count;
    }
    (count - 1) as u128
}
