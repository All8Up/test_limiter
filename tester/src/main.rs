use limiter::Limiter;

fn main() {
    // Current test is fairly simple, run a loop with the limiter and measure errors.
    const FPS_TARGET: u64 = 100;
    const TOTAL_DURATION_SECONDS: u64 = 5;

    // TODO: Currently missing is a do nothing CPU eater to see what impact the limiter has on the overall
    // system.  I.e. do some matrix multiplies in the background or something on each thread.

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
        // Create the limiter.
        let limiter = (pair.1)();

        // Record total time and per wait time.
        let mut deltas = Vec::<std::time::Duration>::with_capacity(total_loops as usize);
        let start = std::time::Instant::now();
        {
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

        println!("Limiter: {} - total time: {}ms (s: {}) - avg: {}ms - min: {}ms - max: {}ms",
            pair.0,
            delta_time.as_millis(),
            delta_time.as_secs_f64(),
            avg,
            min,
            max);
    }
}
