use crate::models::lastfm::Lastfm;
use crate::DatabaseConnection;
use crate::LASTFM_API_KEY;
use log::*;
use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

#[command]
#[description = "Link your lastfm account to your discord account"]
#[example = "HansTrashy"]
#[usage = "*lastfmusername*"]
#[num_args(1)]
pub fn register(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let username = args.single::<String>()?;
    let data = ctx.data.write();
    let mut conn = data
        .get::<DatabaseConnection>()
        .map(|v| v.get().expect("pool error"))
        .ok_or("Could not retrieve the database connection!")?;

    if let Ok(user) = Lastfm::get(&mut *conn, *msg.author.id.as_u64() as i64) {
        let lastfm = Lastfm::update(&mut *conn, user.id, username)?;

        msg.reply(
            &ctx,
            format!("Updated your lastfm username to {}", lastfm.username),
        )?;
    } else {
        let lastfm = Lastfm::create(&mut *conn, *msg.author.id.as_u64() as i64, username)?;

        msg.reply(
            &ctx,
            format!("added {} as your lastfm username!", lastfm.username),
        )?;
    }

    Ok(())
}

#[command]
#[description = "Show your currently playing track"]
#[num_args(0)]
#[bucket = "lastfm"]
pub fn now(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let data = ctx.data.write();
    let mut conn = data
        .get::<DatabaseConnection>()
        .map(|v| v.get().expect("pool error"))
        .ok_or("Could not retrieve the database connection!")?;

    let lastfm = Lastfm::get(&mut *conn, *msg.author.id.as_u64() as i64)?;

    // prepare for the lastfm api
    let url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user={}&api_key={}&format=json",
            lastfm.username,
            *LASTFM_API_KEY);

    let res: serde_json::Value = reqwest::blocking::get(&url)?.json()?;

    // ignore the case where users only played a single title and there is no array
    if let Some(tracks) = res
        .pointer("/recenttracks/track")
        .and_then(|a| a.as_array())
    {
        for t in tracks {
            // here we have a boolean that only ever can be true, otherwise it is non existent, also, it is a string
            if t.pointer("/@attr/nowplaying")
                .and_then(|a| a.as_str())
                .unwrap_or("")
                == "true"
            {
                let content = format!(
                    "Artist: {} - {}",
                    t.pointer("/artist/#text")
                        .and_then(|a| a.as_str())
                        .unwrap_or("Unknown Artist"),
                    t.pointer("/name")
                        .and_then(|a| a.as_str())
                        .unwrap_or("Unknown Title")
                );

                msg.channel_id.send_message(&ctx, |m| m.content(&content))?;
            }
        }
    }

    Ok(())
}

#[command]
#[description = "Show your recent tracks"]
#[num_args(0)]
#[bucket = "lastfm"]
pub fn recent(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let data = ctx.data.write();
    let mut conn = data
        .get::<DatabaseConnection>()
        .map(|v| v.get().expect("pool error"))
        .ok_or("Could not retrieve the database connection!")?;

    let lastfm = Lastfm::get(&mut *conn, *msg.author.id.as_u64() as i64)?;

    // prepare for the lastfm api
    let url = format!("http://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user={}&api_key={}&format=json&limit=10",
            lastfm.username,
            *LASTFM_API_KEY);

    let res: serde_json::Value = reqwest::blocking::get(&url)?.json()?;

    let mut content = String::new();

    // ignore the case where users only played a single title and there is no array
    if let Some(tracks) = res
        .pointer("/recenttracks/track")
        .and_then(|a| a.as_array())
    {
        for t in tracks {
            content.push_str(&format!(
                "Artist: {} - {}\n",
                t.pointer("/artist/#text")
                    .and_then(|a| a.as_str())
                    .unwrap_or("Unknown Artist"),
                t.pointer("/name")
                    .and_then(|a| a.as_str())
                    .unwrap_or("Unknown Title"),
            ));
        }
    }

    msg.channel_id.send_message(&ctx, |m| m.content(&content))?;

    Ok(())
}

#[command]
#[description = "Show your top artists"]
#[usage = "(all|7d|1m|3m|6m|12m)"]
#[example = "3m"]
#[min_args(0)]
#[max_args(1)]
#[bucket = "lastfm"]
pub fn artists(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let period = match args.rest() {
        "all" => "overall",
        "7d" => "7day",
        "1m" => "1month",
        "3m" => "3month",
        "6m" => "6month",
        "12m" => "12month",
        _ => "overall",
    };

    let data = ctx.data.write();
    let mut conn = data
        .get::<DatabaseConnection>()
        .map(|v| v.get().expect("pool error"))
        .ok_or("Could not retrieve the database connection!")?;

    let lastfm = Lastfm::get(&mut *conn, *msg.author.id.as_u64() as i64)?;

    // prepare for the lastfm api
    let url = format!("http://ws.audioscrobbler.com/2.0/?method=user.gettopartists&user={}&api_key={}&format=json&limit=10&period={}",
            lastfm.username,
            *LASTFM_API_KEY,
            period);

    let res: serde_json::Value = reqwest::blocking::get(&url)?.json()?;

    let mut content = String::new();

    if let Some(artists) = res.pointer("/topartists/artist").and_then(|a| a.as_array()) {
        for a in artists {
            content.push_str(&format!(
                "Rank: {} | {}\n",
                a.pointer("/@attr/rank")
                    .and_then(|a| a.as_str())
                    .unwrap_or("Unknown Rank"),
                a.pointer("/name")
                    .and_then(|a| a.as_str())
                    .unwrap_or("Unknown Artist"),
            ));
        }
    }

    msg.channel_id.send_message(&ctx, |m| m.content(&content))?;

    Ok(())
}

#[command]
#[description = "Show your top albums"]
#[usage = "(all|7d|1m|3m|6m|12m)"]
#[example = "3m"]
#[min_args(0)]
#[max_args(1)]
#[bucket = "lastfm"]
pub fn albums(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let period = match args.rest() {
        "all" => "overall",
        "7d" => "7day",
        "1m" => "1month",
        "3m" => "3month",
        "6m" => "6month",
        "12m" => "12month",
        _ => "overall",
    };

    let data = ctx.data.write();
    let mut conn = data
        .get::<DatabaseConnection>()
        .map(|v| v.get().expect("pool error"))
        .ok_or("Could not retrieve the database connection!")?;

    let lastfm = Lastfm::get(&mut *conn, *msg.author.id.as_u64() as i64)?;

    // prepare for the lastfm api
    let url = format!("http://ws.audioscrobbler.com/2.0/?method=user.gettopalbums&user={}&api_key={}&format=json&limit=10&period={}",
            lastfm.username,
            *LASTFM_API_KEY,
            period);

    let res: serde_json::Value = reqwest::blocking::get(&url)?.json()?;

    let mut content = String::new();

    if let Some(albums) = res.pointer("/topalbums/album").and_then(|a| a.as_array()) {
        for a in albums {
            content.push_str(&format!(
                "Rank: {} | {}\n",
                a.pointer("/@attr/rank")
                    .and_then(|a| a.as_str())
                    .unwrap_or("Unknown Rank"),
                a.pointer("/name")
                    .and_then(|a| a.as_str())
                    .unwrap_or("Unknown Artist"),
            ));
        }
    }

    msg.channel_id.send_message(&ctx, |m| m.content(&content))?;

    Ok(())
}

#[command]
#[description = "Show your top tracks"]
#[usage = "(all|7d|1m|3m|6m|12m)"]
#[example = "3m"]
#[min_args(0)]
#[max_args(1)]
#[bucket = "lastfm"]
pub fn tracks(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let period = match args.rest() {
        "all" => "overall",
        "7d" => "7day",
        "1m" => "1month",
        "3m" => "3month",
        "6m" => "6month",
        "12m" => "12month",
        _ => "overall",
    };

    info!("period: {:?}", period);

    let data = ctx.data.write();
    let mut conn = data
        .get::<DatabaseConnection>()
        .map(|v| v.get().expect("pool error"))
        .ok_or("Could not retrieve the database connection!")?;

    let lastfm = Lastfm::get(&mut *conn, *msg.author.id.as_u64() as i64)?;

    // prepare for the lastfm api
    let url = format!("http://ws.audioscrobbler.com/2.0/?method=user.gettoptracks&user={}&api_key={}&format=json&limit=10&period={}",
            lastfm.username,
            *LASTFM_API_KEY,
            period);

    let res: serde_json::Value = reqwest::blocking::get(&url)?.json()?;

    let mut content = String::new();

    if let Some(tracks) = res.pointer("/toptracks/track").and_then(|a| a.as_array()) {
        for t in tracks {
            content.push_str(&format!(
                "Rank: {} | Played: {} | {} - {}\n",
                t.pointer("/@attr/rank")
                    .and_then(|a| a.as_str())
                    .unwrap_or("Unknown Rank"),
                t.pointer("/playcount")
                    .and_then(|a| a.as_str())
                    .unwrap_or("-"),
                t.pointer("/artist/name")
                    .and_then(|a| a.as_str())
                    .unwrap_or("Unknown Artist"),
                t.pointer("/name")
                    .and_then(|a| a.as_str())
                    .unwrap_or("Unknown Track"),
            ));
        }
    }

    msg.channel_id.send_message(&ctx, |m| m.content(&content))?;

    Ok(())
}
