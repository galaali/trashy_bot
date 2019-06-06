use crate::models::bank::Bank;
use crate::schema::banks::dsl::*;
use crate::DatabaseConnection;
use diesel::prelude::*;
use serenity::model::channel::ReactionType;
use crate::BlackjackState;
use serenity::{
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
    model::channel::Message,
};
use serenity::prelude::*;
use log::*;

#[command]
#[bucket = "blackjack"]
#[description = "Play a round of Blackjack"]
pub fn blackjack(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let pool = match data.get::<DatabaseConnection>() {
        Some(v) => v.clone(),
        None => {
            let _ = msg
                .channel_id
                .say(&ctx, "Datenbankfehler, bitte informiere einen Moderator!");
            return Ok(());
        }
    };
    let conn: &PgConnection = &pool.get().unwrap();
    let amount_to_bet = match args.single::<i64>() {
        Ok(v) if v > 0 => v,
        Ok(_) => {
            // log
            let _ = msg.channel_id.say(&ctx, "Ungültiger Wetteinsatz!");
            return Ok(());
        }
        Err(_e) => {
            // log
            let _ = msg.channel_id.say(&ctx, "Ungültiger Wetteinsatz!");
            return Ok(());
        }
    };
    let blackjack_state = match data.get::<BlackjackState>() {
        Some(v) => v.clone(),
        None => {
            let _ = msg.reply(&ctx, "Could not retrieve the blackjack state!");
            return Ok(());
        }
    };

    // check if user already owns a bank & has enough balance
    let results = banks
        .filter(user_id.eq(*msg.author.id.as_u64() as i64))
        .load::<Bank>(conn)
        .expect("could not retrieve banks");

    if !results.is_empty() && results[0].amount >= amount_to_bet {
        // remove betted amount

        // create blackjack game message and add it to blackjack state
        let blackjack_msg = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.content("Starting Blackjack game...").reactions(vec![
                    ReactionType::Unicode("👆".to_string()),
                    ReactionType::Unicode("✋".to_string()),
                    ReactionType::Unicode("🌀".to_string()),
                ])
            })
            .expect("Failed to create blackjack message");
        blackjack_state.lock().add_game(
            pool.clone(),
            ctx.clone(),
            *msg.author.id.as_u64(),
            amount_to_bet,
            *blackjack_msg.channel_id.as_u64(),
            *blackjack_msg.id.as_u64(),
        );
    } else {
        let _ = msg.channel_id.say(
            &ctx,
            "Du besitzt entweder keine Bank, oder nicht genügend credits!",
        );
    }
    Ok(())
}