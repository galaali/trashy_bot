pub mod general {
    use serenity::framework::standard::{macros::{command, group, help, check}};
    use crate::commands::{about::*, roll::ROLL_COMMAND, choose::*, xkcd::*, quote::*};

    group!({
        name: "general",
        options: {},
        commands: [about, roll, choose, xkcd, quote],
    });
}

pub mod config {
    use serenity::framework::standard::{macros::{command, group, help, check}};
    use crate::commands::config::*;

    group!({
        name: "config",
        options: {
            prefix: "cfg",
            description: "Config commands",
            default_command: status,
        },
        commands: [status,]
    });
}

pub mod account {
    use serenity::framework::standard::{macros::{command, group, help, check}};
    use crate::commands::account::{general::*, blackjack::*, slot::*};
    group!({
        name: "account",
        options: {
            prefix: "acc",
            description: "Having fun with some games",
            default_command: payday,
        },
        commands: [createaccount, payday, leaderboard, transfer, slot, blackjack]
    });
}

pub mod greenbook {
    use serenity::framework::standard::{macros::{command, group, help, check}};
    use crate::commands::fav::*;
    group!({
        name: "greenbook",
        options: {
            prefix: "fav",
            description: "Saving your favourite messages.",
            default_command: post,
        },
        commands: [post, untagged, add, tags],
    });
}

pub mod rules {
    use serenity::framework::standard::{macros::{command, group, help, check}};
    use crate::commands::rules::*;
    group!({
        name: "rules",
        options: {
            prefix: "rules",
            description: "Rules to be sent by the bot",
        },
        commands: [],
    });
}

pub mod reaction_roles {
    use serenity::framework::standard::{macros::{command, group, help, check}};
    use crate::commands::reaction_roles::*;
    group!({
        name: "reaction_roles",
        options: {
            prefix: "rr",
            description: "Let users easily add roles to themselves with reactions",
            default_command: list,
        },
        commands: [list, create, remove, postgroups],
    });
}