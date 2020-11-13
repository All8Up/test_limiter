use limiter::Limiter;
use crate::DurationSample;

pub fn run(name: &str, limiter: Box<dyn Limiter>, count: usize, delta: std::time::Duration) -> Option<Vec<String>> {
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
