use serenity::{
    all::ActivityData,
    client::{Context, EventHandler},
    model::{
        channel::Reaction,
        gateway::Ready,
        id::{ChannelId, MessageId},
    },
    prelude::GatewayIntents,
    Client,
};

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, c: Context, r: Ready) {
        let name = &r.user.name;
        c.set_activity(Some(ActivityData::custom("5x📌 pins a message!")));
        println!("{} is online!", name);
    }

    async fn reaction_add(&self, c: Context, r: Reaction) {
        // println!("Reaction added: {:?}", r);
        if let Err(e) = handle_react(c, r).await {
            println!("Error: {:?}", e);
        }
    }

    async fn reaction_remove(&self, c: Context, r: Reaction) {
        // println!("Reaction removed: {:?}", r);
        if let Err(e) = handle_react(c, r).await {
            println!("Error: {:?}", e);
        }
    }

    async fn reaction_remove_all(&self, c: Context, c_id: ChannelId, m_id: MessageId) {
        // println!("All reactions removed from message: {:?}", m_id);
        let msg = match c.http.get_message(c_id, m_id).await {
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
    println!("Handling reaction: {:?}", r);

    if r.emoji.unicode_eq("📌") {
        println!("Pin reaction detected!");
        let msg = r.message(&c).await?;
        // println!("Message: {:?}", msg);
        let count = msg
            .reactions
            .iter()
            .find(|r| r.reaction_type.unicode_eq("📌"))
            .map(|r| r.count)
            .unwrap_or_default();
        println!("Pin count: {}", count);
        if count >= 1 {
            msg.pin(&c).await?;
        } else if count < 3 {
            msg.unpin(&c).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Where discord token?");
    let intents = GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Bruh");
    if let Err(why) = client.start().await {
        println!("Error starting client: {:?}", why);
    }
}
