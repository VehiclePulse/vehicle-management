use lapin::{
    BasicProperties, Channel, Confirmation, Connection, ConnectionProperties, options::{BasicPublishOptions}, types::{ShortString},
};
use serde::Serialize;


pub struct RabbitmqPublisher {
    _connection: Connection,
    channel: Channel,
    exchange: String
}

impl RabbitmqPublisher {
    pub async fn new(
        amqp_url: String,
        exchange: String
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let connection = Connection::connect(
            amqp_url.as_str(),
            ConnectionProperties::default().enable_auto_recover()
        ).await?;

        let channel = connection.create_channel().await?;

        channel.confirm_select(Default::default()).await?;

        Ok(Self {
            _connection: connection,
            channel,
            exchange: exchange
        })
    }

    pub async fn publish<T: Serialize>(
        &self,
        routing_key: &str,
        event: &T
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let payload = serde_json::to_vec(event)?;

        let confirmation = self
            .channel
            .basic_publish(
                ShortString::from(self.exchange.clone()), 
                ShortString::from(routing_key), 
                BasicPublishOptions::default(), 
                &payload, 
                BasicProperties::default()
                    .with_content_type("application/json".into())
                    .with_delivery_mode(2)
                )
            .await?
            .await?;

        match confirmation {
            Confirmation::Ack(_) => Ok(()),
            Confirmation::Nack(_) => {
                Err("RabbitMq negatively acknowledged the publication".into())
            },
            Confirmation::NotRequested => {
                Err("Publisher confirmation was not requested".into())
            }
        }
    }
}