use crate::config::MqttConfig;
use crate::error::Blink1Error;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, ConnectionError, Event, MqttOptions};
use std::time::Duration;
use rumqttc::v5::mqttbytes::v5::Packet;
use tokio_stream::wrappers::ReceiverStream;

pub async fn init(config: &MqttConfig) -> Result<ReceiverStream<rumqttc::v5::mqttbytes::v5::Publish>, Blink1Error> {
    let mut mqttoptions = MqttOptions::new("blink1", &config.host, config.port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe("blink1/set", QoS::AtMostOnce)
        .await
        .map_err(Box::new)
        .map_err(Blink1Error::MqttSubscribeError)?;

    let (tx, rx) = tokio::sync::mpsc::channel(10);
    let stream = ReceiverStream::new(rx);

    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(publish))) => {
                    if tx.send(publish).await.is_err() {
                        tracing::warn!("Receiver dropped");
                        break;
                    }
                }
                Ok(_e) => {
                    //tracing::debug!(?e, "Received MQTT event");
                }
                Err(e) => {
                    tracing::warn!("MQTT connection error: {:?}", e);
                }
            }
        }
    });

    Ok(stream)
}
