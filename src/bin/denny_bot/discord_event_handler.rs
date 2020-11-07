use crate::limited_budgetworks_server::utils::add_role_rules_verified;
use crate::twitch_webhook_handler::set_up_twitch_webhooks;
use crate::VERSION;
use serenity::model::prelude::{Reaction, Ready};
use serenity::prelude::{Context, EventHandler};
use crate::test_server::{_reaction_add_test, _reaction_remove_test};
use serenity::model::channel::{Message, Embed};
use serenity::async_trait;
use serenity::utils::Color;
use serenity::model::id::ChannelId;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        add_role_rules_verified(&ctx, &reaction).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        println!("Version: {}", VERSION);

        if let Err(why) = ChannelId(773036830580408330).send_message(&ctx, |m| {
            m
                .embed(|e| {
                    e
                        .author(|a| {
                            a.icon_url(&ready.user.face())
                                .name(&ready.user.name)
                        })
                        .description(format!("\
                        {} is connected!\n\
                        Version: {}
                        ", &ready.user.name, &VERSION))
                        .color(Color::from_rgb(255, 128, 0))
                })
        }).await {
            println!("{}", why)
        };
        // set_up_twitch_webhooks(ctx);
    }
}
