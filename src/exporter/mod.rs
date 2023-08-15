pub trait Exporter {
    // TODO: Implement a struct with config
    fn new() -> Self;
    fn fetch(&self) -> Metric;
}