use crate::models::reaction_role::ReactionRole;
use crate::reaction_roles::State;
use crate::DatabasePool;
use crate::ReactionRolesState;
use log::info;
use serenity::{
    model::{channel::Reaction, channel::ReactionType},
    prelude::*,
};

pub async fn add_role(ctx: Context, add_reaction: Reaction) {
    let data = ctx.data.read().await;
    let mut conn = match data.get::<DatabasePool>() {
        Some(v) => v.get().await.unwrap(),
        None => return,
    };
    let (rr_channel_id, rr_message_ids) = match data.get::<ReactionRolesState>() {
        Some(v) => match *v.lock().await {
            State::Set {
                ref channel_id,
                ref message_ids,
            } => (*channel_id, message_ids.to_owned()),
            State::NotSet => return,
        },
        None => return,
    };
    // check if reaction is on rr message
    if add_reaction.channel_id == rr_channel_id
        && rr_message_ids.contains(add_reaction.message_id.as_u64())
    {
        info!("On correct message reacted!");
        if let ReactionType::Unicode(ref s) = add_reaction.emoji {
            // check if rr registered for this emoji
            let results = ReactionRole::list_by_emoji(&mut *conn, s)
                .await
                .expect("could not get by emojis");

            if !results.is_empty() {
                info!("Found role for this emoji!");
                if let Some(guild) = add_reaction
                    .channel_id
                    .to_channel(&ctx)
                    .await
                    .ok()
                    .and_then(|c| c.guild())
                {
                    if let Ok(mut member) = guild
                        .read()
                        .await
                        .guild_id
                        .member(&ctx, add_reaction.user_id)
                        .await
                    {
                        let _ = member.add_role(&ctx, results[0].role_id as u64);
                    }
                }
            }
        }
    }
}

pub async fn remove_role(ctx: Context, remove_reaction: Reaction) {
    let data = ctx.data.read().await;
    let mut conn = match data.get::<DatabasePool>() {
        Some(v) => v.get().await.unwrap(),
        None => return,
    };
    let (rr_channel_id, rr_message_ids) = match data.get::<ReactionRolesState>() {
        Some(v) => match *v.lock().await {
            State::Set {
                ref channel_id,
                ref message_ids,
            } => (*channel_id, message_ids.to_owned()),
            State::NotSet => return,
        },
        None => return,
    };
    // check if reaction is on rr message
    if remove_reaction.channel_id == rr_channel_id
        && rr_message_ids.contains(remove_reaction.message_id.as_u64())
    {
        info!("On correct message reacted!");
        if let ReactionType::Unicode(ref s) = remove_reaction.emoji {
            // check if rr registered for this emoji
            let results = ReactionRole::list_by_emoji(&mut *conn, s)
                .await
                .expect("could not get by emojis");

            if !results.is_empty() {
                info!("Found role for this emoji!");
                if let Some(guild) = remove_reaction
                    .channel_id
                    .to_channel(&ctx)
                    .await
                    .ok()
                    .and_then(|c| c.guild())
                {
                    if let Ok(mut member) = guild
                        .read()
                        .await
                        .guild_id
                        .member(&ctx, remove_reaction.user_id)
                        .await
                    {
                        let _ = member.remove_role(&ctx, results[0].role_id as u64);
                    }
                }
            }
        }
    }
}
