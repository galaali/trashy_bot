use crate::interaction::wait::Action;
use crate::models::tag::NewTag;
use crate::DatabaseConnection;
use crate::Waiter;
use diesel::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::{
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        help_commands, Args, CommandOptions, DispatchError, HelpBehaviour, StandardFramework,
    },
    model::{
        channel::Message, channel::Reaction, channel::ReactionType, gateway::Ready, id::ChannelId,
        Permissions,
    },
    prelude::*,
    utils::{content_safe, ContentSafeOptions},
};

// Regexes for bad words
lazy_static! {
    static ref BAD_WORDS: Vec<Regex> = {
        vec![
            Regex::new(r"ell[a|e]*").unwrap(),
            Regex::new(r"sex[a|e]*").unwrap(),
        ]
    };
}

mod fav;

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn message(&self, ctx: Context, msg: Message) {
        use crate::schema::tags::dsl::*;
        if msg.is_private() {
            // check if waiting for labels
            let data = ctx.data.lock();
            if let Some(waiter) = data.get::<Waiter>() {
                let mut wait = waiter.lock();
                if let Some(waited_fav_id) = wait.waiting(*msg.author.id.as_u64(), Action::AddTags)
                {
                    let conn = match data.get::<DatabaseConnection>() {
                        Some(v) => v.clone(),
                        None => {
                            let _ = msg.reply("Could not retrieve the database connection!");
                            return;
                        }
                    };

                    // clear old tags for this fav
                    diesel::delete(tags.filter(fav_id.eq(waited_fav_id)))
                        .execute(&*conn.lock())
                        .expect("could not delete tags");

                    let received_tags: Vec<NewTag> = msg
                        .content
                        .split(' ')
                        .map(|t| NewTag::new(waited_fav_id, t.to_string()))
                        .collect();
                    crate::models::tag::create_tags(&*conn.lock(), received_tags);

                    let _ = msg.reply("added the tags!");
                }
            }
        } else {
            // using wordfilter to check messages on guild for bad words
            let mut contains_bad_word = false;
            for r in BAD_WORDS.iter() {
                if r.is_match(&msg.content) {
                    contains_bad_word = true;
                }
            }
            if contains_bad_word {
                let _ = ChannelId::from(553_508_425_745_563_648).send_message(|m| {
                    m.embed(|e| {
                        e.author(|a| {
                            a.name(&msg.author.name)
                                .icon_url(&msg.author.static_avatar_url().unwrap_or_default())
                        })
                        .title("Potenzieller Verstoß gegen die Regeln")
                        .description(&msg.content)
                        .color((0, 120, 220))
                        .footer(|f| {
                            f.text(&format!("{}", &msg.timestamp.format("%d.%m.%Y, %H:%M:%S"),))
                        })
                    })
                });
            }
        }
    }

    fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        match add_reaction.emoji {
            ReactionType::Unicode(ref s) if s == "📗" => fav::add_fav(ctx, add_reaction),
            ReactionType::Unicode(ref s) if s == "🗑" => fav::remove_fav(ctx, add_reaction),
            ReactionType::Unicode(ref s) if s == "🏷" => fav::add_label(ctx, add_reaction),
            _ => (),
        }
    }
}
