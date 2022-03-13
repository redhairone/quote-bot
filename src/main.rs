use serenity::{async_trait, model::prelude::*, prelude::*};

struct Bot {
    database: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Some(quote) = msg.content.strip_prefix("!manaquotes add") {
            let quote = quote.trim();
            let id = sqlx::query!(r#"INSERT INTO quotes ( quote ) VALUES ( ?1 )"#, quote)
                .execute(&self.database)
                .await
                .unwrap()
                .last_insert_rowid();
            let response = format!("Added quote #{} to the list", id);
            msg.channel_id.say(&ctx, response).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");
    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");
    let bot = Bot { database };
    let mut client = Client::builder(&token)
        .event_handler(bot)
        .await
        .expect("Error creating client");
    client.start().await.unwrap();
}
