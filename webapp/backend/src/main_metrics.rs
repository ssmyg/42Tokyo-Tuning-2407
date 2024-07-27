// Using dogstatsd package
// !!!!! Compile Error !!!!!!!!!
use metrics_exporter_dogstatsd::DogStatsdBuilder;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // Set up DogStatsD exporter
    //DogStatsdBuilder::new("127.0.0.1:8125", "my_app")
    DogStatsdBuilder::new("dd-agent:8125", "backend")
        .install()
        .expect("Failed to install DogStatsD exporter");

    // Example: Record some metrics
    metrics::increment_counter!("requests_total", "endpoint" => "/api");
    metrics::gauge!("response_time_ms", 200.0, "endpoint" => "/api");

    // Simulate application running
    loop {
        // Record some example metrics
        metrics::increment_counter!("requests_total", "endpoint" => "/api");
        metrics::histogram!("response_time_ms", 150.0, "endpoint" => "/api");

        sleep(Duration::from_secs(1)).await;
    }
}

// Using StatSD package (Generic)
// !!!!! Compile Error !!!!!!!!!
// use metrics_exporter_statsd::StatsdBuilder;
// use std::time::Duration;
// use tokio::time::sleep;
// use log::{info, error};
// 
// #[tokio::main]
// async fn main() {
//     env_logger::init();
//     info!("Starting application");
// 
//     // Set up StatsD exporter
//     let builder = StatsdBuilder::from("dd-agent", 8125)
//         .with_namespace("my_app");
//     match builder.build() {
//         Ok(exporter) => {
//             if let Err(e) = metrics::set_boxed_recorder(Box::new(exporter)) {
//                 error!("Failed to set global recorder: {}", e);
//                 return;
//             }
//             info!("Successfully installed StatsD exporter");
//         }
//         Err(e) => {
//             error!("Failed to build StatsD exporter: {}", e);
//             return;
//         }
//     }
// 
// //    match StatsdBuilder::from("dd-agent", 8125).install() {
// //    //match StatsdBuilder::from("dd-agent:8125", "my_app").install() {
// //        Ok(_) => info!("Successfully installed StatsD exporter"),
// //        Err(e) => {
// //            error!("Failed to install StatsD exporter: {}", e);
// //            return;
// //        }
// //    }
// 
//     // Example: Record some metrics
//     metrics::increment_counter!("requests_total", "endpoint" => "/api");
//     metrics::gauge!("response_time_ms", 200.0, "endpoint" => "/api");
// 
//     // Simulate application running
//     loop {
//         // Record some example metrics
//         info!("Recording metrics");
//         metrics::increment_counter!("requests_total", "endpoint" => "/api");
//         metrics::histogram!("response_time_ms", 150.0, "endpoint" => "/api");
// 
//         sleep(Duration::from_secs(1)).await;
//     }
// }
