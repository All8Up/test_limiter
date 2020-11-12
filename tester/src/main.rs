use limiter::Limiter;
mod duration_sample;
use duration_sample::DurationSample;

fn main() {
    // Current test is fairly simple, run a loop with the limiter and measure errors.
    const FPS_TARGET: usize = 100;
    const TOTAL_DURATION_SECONDS: usize = 5;

    // TODO: Currently missing is a do nothing CPU eater to see what impact the limiter has on the overall
    // system.  I.e. do some matrix multiplies in the background or something on each thread.  The general
    // idea should be to do a bunch of matrix multiples on one thread per core and when they all complete,
    // run the thread limiter and then repeat.
    // NOTE: We want to keep the tests separate though as the current test is useful on it's own.

    let limiters: [(&str, fn () -> Box<dyn Limiter>); 3] = [
        ("Standard Sleep", limiter::standard_sleep::create),
        ("Time Period Standard Sleep", limiter::time_period_standard_sleep::create),
        ("Time Period With Half Targets", limiter::time_period_close_sleep::create)
    ];

    // General test calcs.
    let loop_delta = std::time::Duration::from_millis(((1.0 / (FPS_TARGET as f32)) * 1000.0) as u64);
    let total_loops = FPS_TARGET * TOTAL_DURATION_SECONDS;

    println!("Total loops: {} at delta: {}", total_loops, loop_delta.as_millis());

    // Loop through each limiter and get the information about it.
    for pair in &limiters {
        if let Some(results) = accuracy_test(pair.0, (pair.1)(), total_loops, loop_delta) {
            for s in results {
                println!("{}", s);
            }
        } else {
            println!("--- error timing limiter ---");
        }
    }
}

fn accuracy_test(name: &str, limiter: Box<dyn Limiter>, count: usize, delta: std::time::Duration) -> Option<Vec<String>> {
    let mut call = DurationSample::default();
    let mut total = DurationSample::default();
    if let Some(process) = perf::process_times::capture(|| {
        total.capture(|| {
            for _ in 0..count {
                call.capture(|| limiter.wait(delta) );
            }
        });
    })
    {
        let duration_secs = delta * count as u32;
        let min = call.min().as_millis();
        let max = call.max().as_millis();
        let avg = call.average().as_millis();
        let total_error = if total.duration() > duration_secs {
            (total.duration().as_secs_f32() / duration_secs.as_secs_f32()).fract() * 100.0
        } else {
            (1.0 - (total.duration().as_secs_f32() / duration_secs.as_secs_f32())).fract() * 100.0
        };

        Some(vec![
            format!("Limiter: {}", name),
            format!(" total time: {}ms (s: {})", total.duration().as_millis(), total.duration().as_secs_f64()),
            format!(" avg: {}ms  min: {}ms  max: {}ms", avg, min, max),
            format!(" total error: {}%", total_error),
            format!(" user time: {}ms  kernel time: {}ms", process.user.as_millis(), process.kernel.as_millis())
        ])
    } else {
        None
    }
}
