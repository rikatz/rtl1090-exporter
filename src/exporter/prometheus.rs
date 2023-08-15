use prometheus_client::registry::Registry;

pub struct PrometheusExporter {
    registry: Registry,
}

impl Exporter for PrometheusExporter {
    fn new() -> PrometheusExporter {
        let mut registry = <Registry>::default();
        PrometheusExporter {
            registry,
        }
    }
}