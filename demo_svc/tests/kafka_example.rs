use std::time::Duration;

use anyhow::{anyhow, Result};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{
    BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer,
};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Header, Headers, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use rdkafka::{ClientConfig, ClientContext, Message, TopicPartitionList};
use rust_demo_commons::test_commons;
use tracing::{debug, info, warn};

fn create_common_client_config(brokers: &str) -> ClientConfig {
    let mut config = ClientConfig::new();
    config
        .set("bootstrap.servers", brokers)
        .set("debug", "broker,topic,msg")
        .set_log_level(RDKafkaLogLevel::Debug);

    config
}

#[tokio::test]
async fn kakfa_simple_example() -> Result<()> {
    test_commons::init_logging();

    debug!("dummy");

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: {} 0x{:08x}", version_s, version_n);

    // Use IPv4 instead of `localhost` to prevent IPv6 connection errors
    let brokers = "127.0.0.1:9092";
    let topic_name = "test";

    produce_events(&brokers, topic_name, 5).await?;

    consume_and_print(&brokers, "test-consumerB", &vec![topic_name]).await;

    Ok(())
}

async fn produce_events(brokers: &str, topic_name: &str, amount: i32) -> Result<()> {
    let producer: &FutureProducer = &create_common_client_config(&brokers)
        .set("message.timeout.ms", "5000")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()?;

    for i in 0..amount {
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

        let delivery = delivery_status.map_err(|(a, b)| anyhow!("{a} topic={}", b.topic()))?;

        info!("Future completed successfully. Result: {:?}", delivery);

        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    Ok(())
}

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

async fn consume_and_print(brokers: &str, group_id: &str, topics: &[&str]) {
    let context = CustomContext;

    let consumer: LoggingConsumer = create_common_client_config(brokers)
        .set("group.id", group_id)
        // .set("enable.partition.eof", "false")
        .set("auto.offset.reset", "earliest")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context)
        .expect("Consumer creation failed");

    info!("=== Subscribing to events...");

    consumer
        .subscribe(&topics.to_vec())
        .expect("Can't subscribe to specified topics");

    let mut msg_count = 0;
    while msg_count < 5 {
        // TODO: add timeout:
        match consumer.recv().await {
            Err(err) => warn!("Kafka error: {}", err),
            Ok(bm) => {
                let payload = match bm.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      bm.key(), payload, bm.topic(), bm.partition(), bm.offset(), bm.timestamp());
                if let Some(headers) = bm.headers() {
                    for header in headers.iter() {
                        info!("  Header {:#?}: {:?}", header.key, header.value);
                    }
                }
                consumer.commit_message(&bm, CommitMode::Async).unwrap();
                msg_count += 1;
            }
        };
    }
}
