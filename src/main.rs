use serenity::{
    client::{Context, EventHandler},
    model::{
        channel::Reaction,
        gateway::{Activity, Ready}, id::{ChannelId, MessageId},
    },
    prelude::GatewayIntents,
    Client,
};

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, c: Context, r: Ready) {
        let name = r.user.name;
        c.set_activity(Activity::playing("5x📌 pins a message!"))
            .await;
        println!("{} is online!", name);
    }

    async fn reaction_add(&self, c: Context, r: Reaction) {
        let emoji = r.emoji.as_data();
        if emoji == "📌" {
            let msg = r.message(c.http.clone()).await.expect("Could not find message");
            for r in &msg.reactions {
                let emoji = r.reaction_type.as_data();
                if emoji == "📌" {
                    let count = r.count;
                    if count >= 5 {
                        msg.pin(c.http.clone()).await.expect("Could not pin message");
                    }
                }
            }
        }
    }

    async fn reaction_remove(&self, c: Context, r: Reaction) {
        let emoji = r.emoji.as_data();
        if emoji == "📌" {
            let msg = r.message(&c.http).await.expect("Could not find message");
            for r in &msg.reactions {
                let emoji = r.reaction_type.as_data();
                if emoji == "📌" {
                    let count = r.count;
                    if count < 3 {
                        msg.unpin(&c.http).await.expect("Could not unpin message");
                    }
                }
            }
        }
    }

    async fn reaction_remove_all(
        &self,
        c: Context,
        c_id: ChannelId,
        m_id: MessageId,
    ) {
        let msg = c.http.get_message(c_id.0, m_id.0).await.expect("Could not find message");
        if msg.pinned {
            msg.unpin(&c.http).await.expect("Could not unpin message");
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Where discord token?");
    let intents = GatewayIntents::GUILD_MESSAGE_REACTIONS | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Bruh");
    if let Err(why) = client.start().await {
        println!("Error starting client: {:?}", why);
    }
}
