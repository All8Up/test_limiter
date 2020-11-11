use limiter::Limiter;

fn main() {
    // Current test is fairly simple, run a loop with the limiter and measure errors.
    const FPS_TARGET: u64 = 100;
    const TOTAL_DURATION_SECONDS: u64 = 5;

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
    let duration_secs = std::time::Duration::from_secs(TOTAL_DURATION_SECONDS);

    println!("Total loops: {} at delta: {}", total_loops, loop_delta.as_millis());

    // Loop through each limiter and get the information about it.
    for pair in &limiters {
        // Create the limiter.
        let limiter = (pair.1)();

        // Record total time and per wait time.
        let mut deltas = Vec::<std::time::Duration>::with_capacity(total_loops as usize);
        let start = std::time::Instant::now();
        {
            // TODO: Need Os level thread times (i.e. kernel/user) measurements to verify if the
            // solution is going to cause power issues.  Aka: Win32 GetThreadTimes.  Call this
            // once a second *after* the first second and only within the duration, otherwise it
            // will be polluted.  Of course on a typical system, virus scanners, background tasks
            // etc will pollute the time anyway but might as well be "reasonably" accurate.

            for _ in 0..total_loops {
                let start = std::time::Instant::now();
                limiter.wait(loop_delta);
                let end = std::time::Instant::now();
                deltas.push(end - start);
            }
        }
        let end = std::time::Instant::now();

        let delta_time = end - start;
        let sum_deltas: u128 = deltas.iter().map(|d| d.as_millis()).sum();
        let min: u128 = deltas.iter().min().unwrap().as_millis();
        let max: u128 = deltas.iter().max().unwrap().as_millis();
        let avg: f32 = sum_deltas as f32 / total_loops as f32;
        let total_error = if delta_time > duration_secs {
            (delta_time.as_secs_f32() / duration_secs.as_secs_f32()).fract() * 100.0
        } else {
            (1.0 - (delta_time.as_secs_f32() / duration_secs.as_secs_f32())).fract() * 100.0
        };

        println!("Limiter: {} - total time: {}ms (s: {}) - avg: {}ms - min: {}ms - max: {}ms - total error: {}%",
            pair.0,
            delta_time.as_millis(),
            delta_time.as_secs_f64(),
            avg,
            min,
            max,
            total_error
        );
    }
}
