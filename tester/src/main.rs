use limiter::Limiter;
mod duration_sample;
use duration_sample::DurationSample;
mod accuracy_test;

fn main() {
    // Current test is fairly simple, run a loop with the limiter and measure errors.
    const FPS_TARGET: usize = 100;
    const TOTAL_DURATION_SECONDS: usize = 5;

    // TODO: Currently missing is a do nothing CPU eater to see what impact the limiter has on the overall
    // system.  I.e. do some matrix multiplies in the background or something on each thread.  The general
    // idea should be to do a bunch of matrix multiples on one thread per core and when they all complete,
    // run the thread limiter and then repeat.
    // NOTE: We want to keep the tests separate though as the current test is useful on it's own.

    let limiters: [(&str, fn () -> Box<dyn Limiter>); 4] = [
        ("Standard Sleep", limiter::standard_sleep::create),
        ("Time Period Standard Sleep", limiter::time_period_standard_sleep::create),
        ("Time Period With Half Targets", limiter::time_period_close_sleep::create),
        ("Time Period With Single Sleep", limiter::time_period_close_sleep_once::create),
    ];

    // General test calcs.
    let loop_delta = std::time::Duration::from_millis(((1.0 / (FPS_TARGET as f32)) * 1000.0) as u64);
    let total_loops = FPS_TARGET * TOTAL_DURATION_SECONDS;

    // Loop through each limiter and get the information about it.
    println!("----- Accuracy -----");
    println!("Total loops: {} at delta: {}", total_loops, loop_delta.as_millis());
    for pair in &limiters {
        if let Some(results) = accuracy_test::run(pair.0, (pair.1)(), total_loops, loop_delta) {
            for s in results {
                println!("{}", s);
            }
        } else {
            println!("--- error timing limiter ---");
        }
    }
}
