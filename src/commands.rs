use crate::messenger::*;
use crate::session::*;

use futures::stream::{Stream, StreamExt};
use crate::{Context, Error};

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
    let mut skill_names = params_holder[0][&u.name].keys().cloned().collect::<Vec<String>>();
    skill_names.sort_by(|a, b|b.cmp(a));
    futures::stream::iter(skill_names)
        .filter(move |name| futures::future::ready(name.starts_with(&partial)))
}

/// キャラクターシートを読み込みます．
#[poise::command(prefix_command)]
pub async fn new(
    ctx: Context<'_>,
    #[description = "スプレッドシートID"] gsheet_id: String,
) -> Result<(), Error> {
    let p_params = load_player_params(gsheet_id);
    let mut params_holder = ctx.data().params_holder.lock().await;

    params_holder.clear();
    params_holder.push(p_params.await);

    let response = format!("`キャラクターシート読み込み`\n完了.");
    ctx.say(response).await?;
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
    let skill_val = params_holder[0][&u.name][&skill_name];
    let response = skill_dice_msg(skill_name, skill_val, bonus, penalty, operator, corr);
    ctx.say(response).await?;
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
    let response = match desire {
        Some(desire) => simple_dice_with_desire_msg(qty, die, desire),
        None => simple_dice_msg(qty, die),
    };
    ctx.say(response).await?;
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
    let response = match desire {
        Some(desire) => simple_dice_with_desire_msg(qty, die, desire),
        None => simple_dice_msg(qty, die),
    };
    ctx.say(response).await?;
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
    let response = insan_realtime_msg();
    ctx.say(response).await?;
    Ok(())
}

/// 狂気の発作(サマリー)を表示します．
#[poise::command(prefix_command, slash_command)]
pub async fn summary(ctx: Context<'_>) -> Result<(), Error> {
    let response = insan_summary_msg();
    ctx.say(response).await?;
    Ok(())
}

/// キャラクターを作成します．
#[poise::command(prefix_command, slash_command)]
pub async fn cm(ctx: Context<'_>) -> Result<(), Error> {
    let response = character_make();
    ctx.say(response).await?;
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
    let before_skill_val = params_holder[0][&u.name][&status_name];
    if operator == "+" {
        
        *params_holder[0].entry(u.name.clone().into()).or_default().entry(status_name.clone()).or_default() = before_skill_val.saturating_add(corr); 
    } else if operator == "-" {
        *params_holder[0].entry(u.name.clone().into()).or_default().entry(status_name.clone()).or_default() = before_skill_val.saturating_sub(corr); 
    } else if operator == "*" {
        *params_holder[0].entry(u.name.clone().into()).or_default().entry(status_name.clone()).or_default() = before_skill_val.saturating_mul(corr); 
    } else if operator == "/" {
        *params_holder[0].entry(u.name.clone().into()).or_default().entry(status_name.clone()).or_default() = before_skill_val.saturating_div(corr); 
    }
    let after_skill_val = params_holder[0][&u.name][&status_name];
    let response = set_status_msg(
        &status_name, 
        before_skill_val, 
        after_skill_val, 
        &operator, corr
    );
    ctx.say(response).await?;
    Ok(())
}