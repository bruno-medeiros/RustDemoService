use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use rust_demo_commons::test_commons;
use std::time::Duration;

use rdkafka::util::get_rdkafka_version;
use tracing::info;

use anyhow::{anyhow, Result};

#[tokio::test]
async fn kakfa_simple_example() -> Result<()> {
    test_commons::init_logging();

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let brokers = "localhost:9092";
    let topic_name = "test";

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()?;

    let i = 1;
    let delivery_status = producer
        .send(
            FutureRecord::to(topic_name)
                .payload(&format!("Message {}", i))
                .key(&format!("Key {}", i))
                .headers(OwnedHeaders::new().insert(Header {
                    key: "header_key",
                    value: Some("header_value"),
                })),
            Duration::from_secs(0),
        )
        .await;

    info!("Future completed. Result: {:?}", delivery_status);

     delivery_status
        .map_err(|(a, b)| anyhow!("{a} - {b:?}"))?;
    
    Ok(())
}
