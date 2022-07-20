use crate::messenger::*;
use crate::session::*;

use futures::stream::{Stream, StreamExt};
use poise::serenity_prelude as serenity;
use crate::{Context, Error};

async fn autocomplete_operator(
    _ctx: Context<'_>,
    _partial: String,
) -> Vec<String> {
    vec!["+".to_string(), "-".to_string(), "*".to_string(), "/".to_string()]
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

/// キャラクターシートの読み込み
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

/// 技能判定を行う
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
    corr: Option<usize>,
    #[description = "ボーナスダイスの数"]
    bonus: Option<usize>,
    #[description = "ペナルティダイスの数"]
    penalty: Option<usize>,
) -> Result<(), Error> {
    let u = ctx.author();
    let params_holder = &*ctx.data().params_holder.lock().await;
    let skill_val = params_holder[0][&u.name][&skill_name];
    let response = skill_dice_msg(skill_name, skill_val, bonus, penalty, operator, corr);
    ctx.say(response).await?;
    Ok(())
}

/// アカウントが作成された日時を表示
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "ユーザー"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{} のアカウントは {} に作成されました.", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// ダイスを振る
#[poise::command(slash_command, prefix_command)]
pub async fn dice(
    ctx: Context<'_>,
    #[description = "ダイスの個数"]
    #[min = 1]
    qty: usize,
    #[description = "ダイスの面"]
    #[min = 1]
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

/// 秘匿ダイスを振る
#[poise::command(slash_command, prefix_command, ephemeral)]
pub async fn sdice(
    ctx: Context<'_>,
    #[description = "ダイスの個数"]
    #[min = 1]
    qty: usize,
    #[description = "ダイスの面"]
    #[min = 1]
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

/// メッセージの内容を表示
#[poise::command(context_menu_command = "Echo", slash_command)]
pub async fn echo(
    ctx: Context<'_>,
    #[description = "表示するメッセージ(リンクまたはIDを入力)"] msg: serenity::Message,
) -> Result<(), Error> {
    ctx.say(&msg.content).await?;
    Ok(())
}

/// 狂気の発作
#[poise::command(slash_command, subcommands("real", "summary"))]
pub async fn insan(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("").await?;
    Ok(())
}

/// 狂気の発作(リアルタイム)
#[poise::command(prefix_command, slash_command)]
pub async fn real(ctx: Context<'_>) -> Result<(), Error> {
    let response = insan_realtime_msg();
    ctx.say(response).await?;
    Ok(())
}

/// 狂気の発作(サマリー)
#[poise::command(prefix_command, slash_command)]
pub async fn summary(ctx: Context<'_>) -> Result<(), Error> {
    let response = insan_summary_msg();
    ctx.say(response).await?;
    Ok(())
}

/// キャラクターを作成する
#[poise::command(prefix_command, slash_command)]
pub async fn cm(ctx: Context<'_>) -> Result<(), Error> {
    let response = character_make();
    ctx.say(response).await?;
    Ok(())
}