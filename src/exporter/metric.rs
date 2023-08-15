// Labels defines the Metric labels for flights dumps
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
struct Labels {
  callsign: String,
  squawk: u16, // squawk is a maximum of 4 integers - the 4-digit squawk (octal representation)
}