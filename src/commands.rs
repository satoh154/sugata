#![allow(unused_mut)]
#![allow(unused_assignments)]

use crate::messenger::*;
use crate::session::*;
use crate::{Context, Error};
use futures::stream::{Stream, StreamExt};
use poise::serenity_prelude as serenity;

async fn autocomplete_operator(
    _ctx: Context<'_>,
    _partial: String,
) -> Vec<String> {
    vec!["+", "-", "*", "/"]
    .iter()
    .map(|name| name.to_string())
    .collect()
}

async fn autocomplete_status(
    _ctx: Context<'_>,
    _partial: String,
) -> Vec<String> {
    vec![
        "STR", "CON", "SIZ", "DEX", "APP", "INT", "POW",
        "EDU", "SAN", "MP", "幸運", "耐久力"
    ]
    .iter()
    .map(|name| name.to_string())
    .collect()
}

async fn autocomplete_skill_name(
    ctx: Context<'_>,
    partial: String,
) -> impl Stream<Item = String> {
    let u = ctx.author();
    let params_holder = &*ctx.data().params_holder.lock().await;
    let skill_names = params_holder[0][&u.name].1.keys().cloned().collect::<Vec<String>>();
    futures::stream::iter(skill_names)
        .filter(move |name| futures::future::ready(name.starts_with(&partial)))
}

/// キャラクターシートを読み込みます．
#[poise::command(prefix_command, slash_command)]
pub async fn load(
    ctx: Context<'_>,
    #[description = "スプレッドシートID"] gsheet_id: String,
) -> Result<(), Error> {
    let p_params = load_player_params(gsheet_id.clone());
    match p_params.await {
        Ok(p) => {
            let mut params_holder = ctx.data().params_holder.lock().await;

            params_holder.clear();
            params_holder.push(p.clone());

            let title = format!("キャラクターシート読み込み");
            let mut desc = format!("");
            for (i, (player_name, params)) in p.iter().enumerate() {
                let character_name = &params.0;
                if i + 1 == p.len() {
                    desc += &format!("{}: **{}**", player_name, character_name);
                } else {
                    desc += &format!("{}: **{}**\n", player_name, character_name);
                }
            }

            ctx.send(|b| {
                b.content("")
                    .embed(|b| b.title(title)
                        .url(format!("https://docs.google.com/spreadsheets/d/{}", gsheet_id))
                        .description(desc)
                        .color(serenity::Colour::DARK_BLUE))
            })
            .await?;
        },
        Err(p) => {
            let title = format!("エラー");
            let desc = p;
            
            ctx.send(|b| {
                b.content("")
                    .embed(|b| b.title(title)
                        .description(desc)
                        .color(serenity::Colour::DARK_RED)
                        .footer(|b| {
                            b.icon_url("https://i.ibb.co/9hS0z52/sugata-unchecked.png").text("occurs at `load`")
                        })
                    )
            })
            .await?;
        }
    };

    Ok(())
}

/// 技能判定を行います．
#[poise::command(slash_command)]
pub async fn skill(
    ctx: Context<'_>,
    #[description = "技能名"]
    #[autocomplete = "autocomplete_skill_name"]
    skill_name: String,
    #[description = "補正演算子"]
    #[autocomplete = "autocomplete_operator"]
    operator: Option<String>,
    #[description = "補正値"]
    #[min = 1]
    #[max = 666]
    corr: Option<usize>,
    #[description = "ボーナスダイスの数"]
    #[max = 666]
    bonus: Option<usize>,
    #[description = "ペナルティダイスの数"]
    #[max = 666]
    penalty: Option<usize>,
) -> Result<(), Error> {
    let u = ctx.author();
    let params_holder = &*ctx.data().params_holder.lock().await;

    let mut title = String::from("エラー");
    let mut desc = String::from("");
    match params_holder.clone().get(0) {
        Some(maps) => {
            match maps.get(&u.name) {
                Some(map) => {
                    match map.1.get(&skill_name) {
                        Some(v) => {
                            if let Some(ope) = operator {
                                if vec!["+", "-", "*", "/"].iter().any(|&k| k==&ope) {
                                    let (title, desc, color) = skill_dice_msg(
                                            skill_name, 
                                            *v, 
                                            bonus, 
                                            penalty, 
                                            Some(ope), 
                                            corr
                                    );
                                    ctx.send(|b| {
                                        b.content("")
                                            .embed(|b| b.title(title)
                                                .description(desc)
                                                .color(color))
                                    })
                                    .await?;
                                    return Ok(())
                                } else {
                                    desc = format!("無効な演算子: **{}** が入力されています．", &ope)
                                }
                            } else {
                                let (title, desc, color) = skill_dice_msg(
                                        skill_name, 
                                        *v, 
                                        bonus, 
                                        penalty, 
                                        operator, 
                                        corr
                                );
                                ctx.send(|b| {
                                    b.content("")
                                        .embed(|b| b.title(title)
                                            .description(desc)
                                            .color(color))
                                })
                                .await?;
                                return Ok(())
                            }
                        },
                        _ => desc = format!("**{}**に該当する技能がありません．", &skill_name)
                    };
                },
                _ => desc = format!(
                    "**{}**のデータは読み込まれていません．\n\
                    ユーザーのキャラクターシートを作成し，`/load`で再ロードしてください．", &u.name)
            };
        },
        _ => desc = format!(
            "キャラクターシートが読み込まれていません．\n\
            `/load`でロードしてください．")
    };
    ctx.send(|b| {
        b.content("")
            .embed(|b| b.title(title)
                .description(desc)
                .color(serenity::Colour::DARK_RED)
                .footer(|b| {
                    b.icon_url("https://i.ibb.co/9hS0z52/sugata-unchecked.png").text("occurs at `skill`")
                })
            )
    })
    .await?;
    Ok(())
}

/// ダイスを振ります．
#[poise::command(slash_command, prefix_command)]
pub async fn dice(
    ctx: Context<'_>,
    #[description = "ダイスの個数"]
    #[min = 1]
    #[max = 666]
    qty: usize,
    #[description = "ダイスの面"]
    #[min = 1]
    #[max = 666]
    die: usize,
    #[description = "目標値"]
    desire: Option<usize>,
) -> Result<(), Error> {
    let (title, desc, color) = match desire {
        Some(desire) => simple_dice_with_desire_msg(qty, die, desire),
        None => simple_dice_msg(qty, die),
    };
    ctx.send(|b| {
        b.content("")
            .embed(|b| b.title(title)
                .description(desc)
                .color(color))
    })
    .await?;
    Ok(())
}

/// 秘匿ダイスを振ります．
#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn sdice(
    ctx: Context<'_>,
    #[description = "ダイスの個数"]
    #[min = 1]
    #[max = 666]
    qty: usize,
    #[description = "ダイスの面"]
    #[min = 1]
    #[max = 666]
    die: usize,
    #[description = "目標値"]
    desire: Option<usize>,
) -> Result<(), Error> {
    let (title, desc, color) = match desire {
        Some(desire) => simple_dice_with_desire_msg(qty, die, desire),
        None => simple_dice_msg(qty, die),
    };
    ctx.send(|b| {
        b.content("")
            .embed(|b| b.title(title)
                .description(desc)
                .color(color))
    })
    .await?;
    Ok(())
}

/// 狂気の発作を表示します．
#[poise::command(slash_command, subcommands("real", "summary"))]
pub async fn insan(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("").await?;
    Ok(())
}

/// 狂気の発作(リアルタイム)を表示します．
#[poise::command(prefix_command, slash_command)]
pub async fn real(ctx: Context<'_>) -> Result<(), Error> {
    let (title, desc, color) = insan_realtime_msg();
    ctx.send(|b| {
        b.content("")
            .embed(|b| b.title(title)
                .description(desc)
                .color(color))
    })
    .await?;
    Ok(())
}

/// 狂気の発作(サマリー)を表示します．
#[poise::command(prefix_command, slash_command)]
pub async fn summary(ctx: Context<'_>) -> Result<(), Error> {
    let (title, desc, color) = insan_summary_msg();
    ctx.send(|b| {
        b.content("")
            .embed(|b| b.title(title)
                .description(desc)
                .color(color))
    })
    .await?;
    Ok(())
}

/// キャラクターを作成します．
#[poise::command(prefix_command, slash_command)]
pub async fn make(ctx: Context<'_>) -> Result<(), Error> {
    let (title, desc, color) = character_make();
    ctx.send(|b| {
        b.content("")
            .embed(|b| b.title(title)
                .description(desc)
                .color(color))
    })
    .await?;
    Ok(())
}

/// ステータスを修正します．
#[poise::command(prefix_command, slash_command)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "ステータス名"]
    #[autocomplete = "autocomplete_status"]
    status_name: String,
    #[description = "補正演算子"]
    #[autocomplete = "autocomplete_operator"]
    operator: String,
    #[description = "補正値"]
    #[min = 1]
    #[max = 666]
    corr: usize,
) -> Result<(), Error> {
    let u = ctx.author();
    let mut params_holder = ctx.data().params_holder.lock().await;
    let mut title = String::from("エラー");
    let mut desc = String::from("");
    match params_holder.clone().get(0) {
        Some(maps) => {
            match maps.get(&u.name) {
                Some(map) => {
                    match map.1.get(&status_name) {
                        Some(v) => {
                            if vec!["+", "-", "*", "/"].iter().any(|&k| k==&operator) {
                                *params_holder[0]
                                    .entry(u.name.clone().into())
                                    .or_default()
                                    .1
                                    .entry(status_name.clone())
                                    .or_default() = if operator == "+" {
                                            v.saturating_add(corr) 
                                        } else if operator == "-" {
                                            v.saturating_sub(corr) 
                                        } else if operator == "*" {
                                            v.saturating_mul(corr) 
                                        } else if operator == "/" {
                                            v.saturating_div(corr) 
                                        } else {
                                            v.saturating_add(0) 
                                        };
                                let after_skill_val = params_holder[0][&u.name].1[&status_name];
                                let (title, desc, color) = set_status_msg(
                                    &status_name, 
                                    *v, 
                                    after_skill_val, 
                                    &operator, corr
                                );
                                ctx.send(|b| {
                                    b.content("")
                                        .embed(|b| b.title(title)
                                            .description(desc)
                                            .color(color))
                                })
                                .await?;
                                return Ok(())
                            } else {
                                desc = format!("無効な演算子: **{}** が入力されています．", &operator)
                            }
                        },
                        _ => desc = format!("**{}**に該当する技能がありません．", &status_name)
                    };
                },
                _ => desc = format!(
                    "**{}**のデータは読み込まれていません．\n\
                    ユーザーのキャラクターシートを作成し，`/load`で再ロードしてください．", &u.name)
            };
        },
        _ => desc = format!(
            "キャラクターシートが読み込まれていません．\n\
            `/load`でロードしてください．")
    };
    ctx.send(|b| {
        b.content("")
            .embed(|b| b.title(title)
                .description(desc)
                .color(serenity::Colour::DARK_RED)
                .footer(|b| {
                    b.icon_url("https://i.ibb.co/9hS0z52/sugata-unchecked.png").text("occurs at `set`")
                })
            )
    })
    .await?;
    Ok(())
}

/// 現在のステータスを表示します．
#[poise::command(prefix_command, slash_command)]
pub async fn show(
    ctx: Context<'_>,
    #[description = "プレイヤー"] player: Option<serenity::User>,
) -> Result<(), Error> {
    let u = player.as_ref().unwrap_or_else(|| ctx.author());
    let mut params_holder = ctx.data().params_holder.lock().await;
    let mut title = String::from("エラー");
    let mut desc = String::from("");
    match params_holder.clone().get(0) {
        Some(maps) => {
            match maps.get(&u.name) {
                Some(map) => {
                    let (title, desc, color) = get_status_msg(&u.name, &map.0, &map.1);
                    ctx.send(|b| {
                        b.content("")
                            .embed(|b| b.title(title)
                                .description(desc)
                                .color(color))
                    })
                    .await?;
                    return Ok(())
                },
                _ => desc = format!(
                    "**{}**のデータは読み込まれていません．\n\
                    ユーザーのキャラクターシートを作成し，`/load`で再ロードしてください．", &u.name)
            }
        },
        _ => desc = format!(
            "キャラクターシートが読み込まれていません．\n\
            `/load`でロードしてください．")
    }
    ctx.send(|b| {
        b.content("")
            .embed(|b| b.title(title)
                .description(desc)
                .color(serenity::Colour::DARK_RED)
                .footer(|b| {
                    b.icon_url("https://i.ibb.co/9hS0z52/sugata-unchecked.png").text("occurs at `show`")
                })
            )
    })
    .await?;
    Ok(())

}