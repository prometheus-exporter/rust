# Prometheus Exporter using Rust

## Build
 - debug
   - `cargo build`
 - release
   - `cargo build --release`

<br/>

## Run
 - debug
   - `./target/debug/prometheus-exporter 10000 /metrics`
 - release
   - `./target/release/prometheus-exporter 10000 /metrics`
 - run `127.0.0.1:10000/metrics` in your browser
