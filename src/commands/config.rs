use crate::models::server_config::ServerConfig;
use crate::DatabaseConnection;
use serde::{Deserialize, Serialize};
use serenity::model::gateway::Activity;
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

#[command]
#[num_args(0)]
#[allowed_roles("Mods")]
pub fn status(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    ctx.set_presence(Some(Activity::listening("$help")), OnlineStatus::Online);
    Ok(())
}

// Keep every setting optional and use reasonable defaults
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Guild {
    pub modlog_channel: Option<u64>,
    pub mute_role: Option<u64>,
    pub userlog_channel: Option<u64>,
}

#[command]
#[num_args(0)]
#[allowed_roles("Mods")]
pub fn show_config(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let mut data = ctx.data.write();
    let mut conn = match data.get::<DatabaseConnection>() {
        Some(v) => v.get().unwrap(),
        None => {
            let _ = msg.reply(&ctx, "Could not retrieve the database connection!");
            return Ok(());
        }
    };

    if let Some(server_id) = msg.guild_id {
        let server_config = ServerConfig::get(&mut *conn, *server_id.as_u64() as i64);

        if let Ok(server_config) = server_config {
            let _ = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.description(format!("{:?}", &server_config))
                        .color((0, 120, 220))
                })
            });
        } else {
            let _ = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.description("config for this server is not available")
                        .color((255, 0, 0))
                })
            });
        }
    }

    Ok(())
}

#[command]
#[num_args(1)]
#[allowed_roles("Mods")]
pub fn set_modlog(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let modlog_channel = args.parse::<u64>()?;
    let mut data = ctx.data.write();
    let mut conn = match data.get::<DatabaseConnection>() {
        Some(v) => v.get().unwrap(),
        None => {
            let _ = msg.reply(&ctx, "Could not retrieve the database connection!");
            return Ok(());
        }
    };

    if let Some(server_id) = msg.guild_id {
        match ServerConfig::get(&mut *conn, *server_id.as_u64() as i64) {
            Ok(mut config) => {
                let mut old_guild_config: Guild =
                    serde_json::from_value(config.config.take()).unwrap();

                old_guild_config.modlog_channel = Some(modlog_channel);

                let updated_config = ServerConfig::update(
                    &mut *conn,
                    *server_id.as_u64() as i64,
                    serde_json::to_value(old_guild_config).unwrap(),
                )?;

                let _ = msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(format!("{:?}", &updated_config))
                            .color((0, 120, 220))
                    })
                });
            }
            Err(e) => {
                let mut guild_config = Guild::default();

                guild_config.modlog_channel = Some(modlog_channel);

                let inserted_config = ServerConfig::create(
                    &mut *conn,
                    *server_id.as_u64() as i64,
                    serde_json::to_value(guild_config).unwrap(),
                )?;

                let _ = msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(format!("{:?}", &inserted_config))
                            .color((0, 120, 220))
                    })
                });
            }
        }
    }

    Ok(())
}

#[command]
#[num_args(1)]
#[allowed_roles("Mods")]
pub fn set_userlog(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let userlog_channel = args.parse::<u64>()?;
    let mut data = ctx.data.write();
    let mut conn = match data.get::<DatabaseConnection>() {
        Some(v) => v.get().unwrap(),
        None => {
            let _ = msg.reply(&ctx, "Could not retrieve the database connection!");
            return Ok(());
        }
    };

    if let Some(server_id) = msg.guild_id {
        match ServerConfig::get(&mut *conn, *server_id.as_u64() as i64) {
            Ok(mut config) => {
                let mut old_guild_config: Guild =
                    serde_json::from_value(config.config.take()).unwrap();

                old_guild_config.userlog_channel = Some(userlog_channel);

                let inserted_config = ServerConfig::update(
                    &mut *conn,
                    *server_id.as_u64() as i64,
                    serde_json::to_value(old_guild_config).unwrap(),
                )?;

                let _ = msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(format!("{:?}", &inserted_config))
                            .color((0, 120, 220))
                    })
                });
            }
            Err(e) => {
                let mut guild_config = Guild::default();

                guild_config.modlog_channel = Some(userlog_channel);

                let inserted_config = ServerConfig::create(
                    &mut *conn,
                    *server_id.as_u64() as i64,
                    serde_json::to_value(guild_config).unwrap(),
                )?;

                let _ = msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(format!("{:?}", &inserted_config))
                            .color((0, 120, 220))
                    })
                });
            }
        }
    }

    Ok(())
}

#[command]
#[num_args(1)]
#[allowed_roles("Mods")]
pub fn set_muterole(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let mute_role = args.parse::<u64>()?;
    let mut data = ctx.data.write();
    let mut conn = match data.get::<DatabaseConnection>() {
        Some(v) => v.get().unwrap(),
        None => {
            let _ = msg.reply(&ctx, "Could not retrieve the database connection!");
            return Ok(());
        }
    };

    if let Some(server_id) = msg.guild_id {
        match ServerConfig::get(&mut *conn, *server_id.as_u64() as i64) {
            Ok(mut config) => {
                let mut old_guild_config: Guild =
                    serde_json::from_value(config.config.take()).unwrap();

                old_guild_config.mute_role = Some(mute_role);

                let inserted_config = ServerConfig::update(
                    &mut *conn,
                    *server_id.as_u64() as i64,
                    serde_json::to_value(old_guild_config).unwrap(),
                )?;

                let _ = msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(format!("{:?}", &inserted_config))
                            .color((0, 120, 220))
                    })
                });
            }
            Err(e) => {
                let mut guild_config = Guild::default();

                guild_config.mute_role = Some(mute_role);

                let inserted_config = ServerConfig::create(
                    &mut *conn,
                    *server_id.as_u64() as i64,
                    serde_json::to_value(guild_config).unwrap(),
                )?;

                let _ = msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(format!("{:?}", &inserted_config))
                            .color((0, 120, 220))
                    })
                });
            }
        }
    }

    Ok(())
}
