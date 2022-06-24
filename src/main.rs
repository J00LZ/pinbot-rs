use serenity::{
    client::{Context, EventHandler},
    model::{
        channel::Reaction,
        gateway::{Activity, Ready},
        id::{ChannelId, MessageId},
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
        if let Err(e) = handle_react(c, r).await {
            println!("Error: {:?}", e);
        }
    }

    async fn reaction_remove(&self, c: Context, r: Reaction) {
        if let Err(e) = handle_react(c, r).await {
            println!("Error: {:?}", e);
        }
    }

    async fn reaction_remove_all(&self, c: Context, c_id: ChannelId, m_id: MessageId) {
        let msg = match c.http.get_message(c_id.0, m_id.0).await {
            Ok(m) => m,
            Err(e) => {
                println!("Error: {:?}", e);
                return;
            }
        };
        if msg.pinned {
            if let Err(e) = msg.unpin(&c.http).await {
                println!("Error: {:?}", e);
            }
        }
    }
}

async fn handle_react(c: Context, r: Reaction) -> Result<(), serenity::Error> {
    let emoji = r.emoji.as_data();
    if emoji == "📌" {
        let msg = r.message(c.http.clone()).await?;
        for r in &msg.reactions {
            let emoji = r.reaction_type.as_data();
            if emoji == "📌" {
                let count = r.count;
                if count >= 5 {
                    msg.pin(c.http.clone()).await?;
                } else if count < 3 {
                    msg.unpin(&c.http).await?;
                }
            }
        }
    }
    Ok(())
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
