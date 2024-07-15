use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use rdkafka::Message;
use std::env::args;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use uuid::Uuid;

fn create_producer(bootstrap_server: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_server)
        .set("queue.buffering.max.ms", "0")
        .create()
        .expect("Failed to create client")
}

fn create_consumer(bootstrap_server: &str) -> StreamConsumer {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_server)
        .set("enable.partition.eof", "false")
        // We'll give each session its own (unique) consumer group id,
        // so that each session will receive all messages
        .set("group.id", format!("chat-{}", Uuid::new_v4()))
        .create()
        .expect("Failed to create client")
}

#[tokio::main]
async fn main() {
    // Creates a producer, reading the bootstrap server from the first command-line argument
    // or defaulting to localhost:9092
    let producer = create_producer(
        &args()
            .skip(1)
            .next()
            .unwrap_or("localhost:9092".to_string()),
    );

    // create the consumer
    let consumer = create_consumer(
        &args()
            .skip(1)
            .next()
            .unwrap_or("localhost:9092".to_string()),
    );
    // subscribe to our topic
    consumer.subscribe(&["chat"]).unwrap();

    let mut stdout = tokio::io::stdout();
    stdout.write(b"Welcome to Kafka chat!\n").await.unwrap();

    let mut input_lines = BufReader::new(tokio::io::stdin()).lines();

    stdout.write(b"Please enter your name: ").await.unwrap();
    stdout.flush().await.unwrap();
    let name = input_lines.next_line().await.unwrap().unwrap();

    loop {
        stdout.write(b"> ").await.unwrap();
        stdout.flush().await.unwrap();

        tokio::select! {
            message = consumer.recv() => {
                let message  = message.expect("Failed to read message").detach();
                let key = message.key().ok_or_else(|| "no key for message").unwrap();
                if key == name.as_bytes() {
                    continue;
                }
                let payload = message.payload().ok_or_else(|| "no payload for message").unwrap();
                stdout.write(b"\t").await.unwrap();
                stdout.write(key).await.unwrap();
                stdout.write(b": ").await.unwrap();
                stdout.write(payload).await.unwrap();
                stdout.write(b"\n").await.unwrap();
            }
            line = input_lines.next_line() => {
                match line {
                    Ok(Some(line)) => {
                        producer.send(FutureRecord::<String, _>::to("chat")
                        .key(&name)
                          .payload(&line), Timeout::Never)
                            .await
                            .expect("Failed to produce");
                    }
                    _ => break,
                }
            }
        }
    }
}
