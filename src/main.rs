use serenity::cache::FromStrAndCache;
use serenity::{async_trait, model::guild::Role, model::prelude::*, prelude::*};
use std::time::SystemTime;

struct Bot {
    database: sqlx::SqlitePool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        //if msg
        //    .author
        //    .has_role(
        //        &ctx,
        //        msg.guild_id.unwrap(),
        //        Role::from_str(&ctx, "@Static").await.unwrap().id,
        //    )
        //    .await
        //    .unwrap()
        //{
        //    println!("Success!");
        //}
        if let Some(quote) = msg.content.strip_prefix("!manaquotes add") {
            let quote = quote.trim();
            let id = sqlx::query!(r#"INSERT INTO quotes ( quote ) VALUES ( ?1 )"#, quote)
                .execute(&self.database)
                .await
                .unwrap()
                .last_insert_rowid();
            let response = format!("Added quote #{} to the list", id);
            msg.channel_id.say(&ctx, response).await.unwrap();
        } else if let Some(id) = msg.content.strip_prefix("!manaquotes remove") {
            let id = id.trim().parse::<i64>().unwrap();
            sqlx::query!("DELETE FROM quotes WHERE rowid = ?", id)
                .execute(&self.database)
                .await
                .unwrap();
            let response = format!("Removed quote from the list");
            msg.channel_id.say(&ctx, response).await.unwrap();
        } else if let Some(_) = msg.content.strip_prefix("!manaquotes length") {
            let (column_length,): (i64,) = sqlx::query_as("SELECT COUNT(quote) FROM quotes")
                .fetch_one(&self.database)
                .await
                .unwrap();
            let response = format!(r#"Mana has {} quotes saved."#, column_length);
            msg.channel_id.say(&ctx, response).await.unwrap();
        } else if let Some(_) = msg.content.strip_prefix("!manaquotes random") {
            let (column_length,): (i64,) = sqlx::query_as("SELECT COUNT(quote) FROM quotes")
                .fetch_one(&self.database)
                .await
                .unwrap();
            let mut rand_rowid: i64 = (SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                % column_length as u64)
                .try_into()
                .unwrap();
            let quotes = sqlx::query!(r#"SELECT quote FROM quotes"#)
                .fetch_all(&self.database)
                .await
                .unwrap();
            let mut actual_quotes = Vec::new();
            for quote in quotes.iter() {
                actual_quotes.push(quote.quote.clone());
            }
            let response = format!(r#""{}""#, actual_quotes.get(rand_rowid as usize).unwrap());
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
