pub trait Limiter {
    fn wait(&self, time: std::time::Duration);
}
