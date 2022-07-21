use std::collections::HashMap;

use rand::{thread_rng, Rng};

fn get_dice_val(qty: usize, die:usize, bonus: Option<usize>, penalty: Option<usize>) -> usize {
    let mut rng = thread_rng();
    let mut res_vec: Vec<usize> = Vec::new(); 
    let mut corr_dice_vec: Vec<usize> = Vec::new();
    for _i in 1..=qty {
        res_vec.push(rng.gen_range(1..=die));
    }
    let mut dice_res = res_vec.iter().sum();

    let bonus = bonus.unwrap_or(0);
    let penalty = penalty.unwrap_or(0);

    if bonus > penalty {
        let corr_dice_num = bonus - penalty;
        for _i in 1..=corr_dice_num {
            corr_dice_vec.push(rng.gen_range(1..=10));
        }
        let corr_dice = corr_dice_vec.iter().min().unwrap();
        if dice_res > corr_dice * 10 {
            dice_res = dice_res - (dice_res - (dice_res % 10 + corr_dice * 10));
        } else {()}
    } else if penalty > bonus {
        let corr_dice_num = penalty - bonus;
        for _i in 1..=corr_dice_num {
            corr_dice_vec.push(rng.gen_range(1..=10));
        }
        let corr_dice = corr_dice_vec.iter().max().unwrap();
        if dice_res < corr_dice * 10 {
            dice_res = dice_res + ((dice_res % 10 + corr_dice * 10) - dice_res);
        } else {()}
    } else {()}

    dice_res
}

pub fn simple_dice_msg(qty: usize, die: usize) -> String {
    let dice_res = get_dice_val(qty, die, Some(0), Some(0));
    let msg = format!("`ダイスロール`\n{}d{} => {}", qty, die, dice_res);

    msg
}

pub fn simple_dice_with_desire_msg(qty: usize, die: usize, desire: usize) -> String {
    let dice_res = get_dice_val(qty, die, Some(0), Some(0));

    let judge =
        if dice_res == 1 {
            "**Critical**"
        } else if dice_res <= desire / 5 {
            "**Extreme**"
        } else if dice_res <= desire / 2 {
            "**Hard**"
        } else if dice_res <= desire {
            "**Regular**"
        } else if dice_res >= 96 {
            "**Failure(Fumble)**"
        } else {
            "**Failure**"
        };

    let msg = format!("`ダイスロール / 目標値: {}`\n{}d{} => {}: {}", desire, qty, die, dice_res, judge);

    msg
}

pub fn skill_dice_msg(
    skill_name: String, 
    mut desire: usize, 
    bonus: Option<usize>, 
    penalty: Option<usize>, 
    operator: Option<String>, 
    corr_val: Option<usize>
) -> String {    
    if let Some(o) = &operator {
        if let Some(c) = corr_val {
            if o == "+" {
                desire += c
            } else if o == "-" {
                desire -= c
            } else if o == "*" {
                desire *= c 
            } else if o == "/" {
                desire /= c
            };
        }
    }

    let dice_res = get_dice_val(1, 100, bonus, penalty);

    let judge =
        if dice_res == 1 {
            "**Critical**"
        } else if dice_res <= desire / 5 {
            "**Extreme**"
        } else if dice_res <= desire / 2 {
            "**Hard**"
        } else if dice_res <= desire {
            "**Regular**"
        } else if dice_res >= 96 {
            "**Failure(Fumble)**"
        } else {
            "**Failure**"
        };

    let bonus = bonus.unwrap_or(0);
    let penalty = penalty.unwrap_or(0);
    let operator = operator.unwrap_or("±".to_string());
    let corr_val = corr_val.unwrap_or(0);
    let msg = if bonus > penalty {
        format!("`技能ロール / {}: {}({}{}, b{})`\n=> {}: {}", skill_name, desire, operator, corr_val, bonus, dice_res, judge)
    } else if penalty > bonus {
        format!("`技能ロール / {}: {}({}{}, p{})`\n=> {}: {}", skill_name, desire, operator, corr_val, penalty, dice_res, judge)
    } else {
        format!("`技能ロール / {}: {}({}{})`\n=> {}: {}", skill_name, desire, operator, corr_val, dice_res, judge)
    };

    msg
}

pub fn insan_realtime_msg() -> String {
    let mut rng = thread_rng();
    let insan_num:usize = rng.gen_range(1..=10);
    let round:usize = rng.gen_range(1..=10);

    let insan_msg = match insan_num {
        1 => format!(
                "**健忘症: {}ラウンド**\n\
                最後にいた安全な場所の記憶以降に起きた出来事を忘れてしまう", round),
        2 => format!(
                "**身体的症状: {}ラウンド**\n\
                狂気によって視覚や聴覚がおかしくなったり，手足が動かなくなったりする", round),
        3 => format!(
                "**暴力衝動: {}ラウンド**\n\
                視界が赤く染まり，暴力と破壊の衝動に駆られる．", round),
        4 => format!(
                "**偏執症: {}ラウンド**\n\
                周りの全てを疑い誰も信用せず，近づくものは的だと認識する．", round),
        5 => format!(
                "**重要な存在: {}ラウンド**\n\
                その場にいた人あるいはモノを自分にとって重要な存在だと思い込む．", round),
        6 => format!(
                "**失神: {}ラウンド**\n\
                失神し，その場に倒れ込む．", round),
        7 => format!(
                "**パニックになって逃亡する: {}ラウンド**\n\
                あらゆる手段を用いてその場から逃走しようとする．", round),
        8 => format!(
                "**身体的ヒステリーもしくは感情爆発: {}ラウンド**\n\
                笑ったり泣いたり叫んだりし続けて行動ができなくなる", round),
        9 => format!(
                "**恐怖症: {}ラウンド**\n\
                恐怖症を発症し，そこに畏怖すべき存在があると思い込む．", round),
        10 => format!(
                "**マニア: {}ラウンド**\n\
                新しいマニアに陥り，今ある状況でそのマニアについて没頭してしまう．", round),
        _ => format!("")
    };
    let msg = format!("`狂気の発作(リアルタイム)`\n{}", insan_msg);
    msg
}

pub fn insan_summary_msg() -> String {
    let mut rng = thread_rng();
    let insan_num:usize = rng.gen_range(1..=10);

    let insan_msg = match insan_num {
        1 => format!(
                "**健忘症**\n\
                意識を取り戻すと見知らぬ場所に移動しており，自分が何者かも忘れている．"),
        2 => format!(
                "**盗難**\n\
                意識を取り戻すと所持品が無くなっていることに気付く．"),
        3 => format!(
                "**暴行**\n\
                時間後に意識を取り戻すと怪我をしており，\n\
                耐久力が発作を起こす前の半分に減少している"),
        4 => format!(
                "**暴力**\n\
                周囲に暴力を振るっている．\n\
                もしかすると物を壊したり，人を傷つけているかもしれない．"),
        5 => format!(
                "**重要な人々**\n\
                バックストーリーの重要な人々に対して「最善」と思える行動を行う．\n\
                ただし，一方的な考えによるものである．"),
        6 => format!(
                "**イデオロギー/信念**\n\
                バックストーリーのイデオロギー/信念に基づく行為をする．\n\
                その姿は感情的で狂気じみており，見た人は恐怖を感じるかもしれない．"),
        7 => format!(
                "**収容**\n\
                意識を取り戻すと精神医療施設や警察の留置所に収容されている．"),
        8 => format!(
                "**パニック**\n\
                狂気中に逃走を図ったため，意識を取り戻すと遠く離れた場所にいることに気付く．\n\
                もしかするとチケットを買わず徒歩で山奥や荒野を延々と歩き続けているかもしれない．"),
        9 => format!(
                "**恐怖症**\n\
                新たに恐怖症を獲得する．"),
        10 => format!(
                "**マニア**\n\
                新たにマニアを獲得する．"),
        _ => format!("")
    };
    let msg = format!("`狂気の発作(サマリ)`\n{}", insan_msg);
    msg
}

pub fn character_make() -> String {
    let mut status:HashMap<&str, String> = HashMap::new();
    status.insert("STR",(5 * get_dice_val(3, 6, Some(0), Some(0))).to_string());
    status.insert("CON",(5 * get_dice_val(3, 6, Some(0), Some(0))).to_string());
    status.insert("SIZ", (5 * (get_dice_val(2, 6, Some(0), Some(0)) + 6)).to_string());
    status.insert("DEX",(5 * get_dice_val(3, 6, Some(0), Some(0))).to_string());
    status.insert("APP",(5 * get_dice_val(3, 6, Some(0), Some(0))).to_string());
    status.insert("INT", (5 * (get_dice_val(2, 6, Some(0), Some(0)) + 6)).to_string());
    status.insert("POW",(5 * get_dice_val(3, 6, Some(0), Some(0))).to_string());
    status.insert("EDU", (5 * (get_dice_val(2, 6, Some(0), Some(0)) + 6)).to_string());
    status.insert("SAN", status["POW"].clone());
    status.insert("MP", (status["POW"].parse::<usize>().unwrap() / 5).to_string());
    status.insert("幸運",(5 * get_dice_val(3, 6, Some(0), Some(0))).to_string());
    status.insert("耐久力", ((status["CON"].parse::<usize>().unwrap() + status["SIZ"].parse::<usize>().unwrap()) / 10).to_string());
    
    let atk = status["STR"].parse::<usize>().unwrap() + status["SIZ"].parse::<usize>().unwrap();

    if atk <= 64 {
        status.insert("db", String::from("-2"));
        status.insert("ビルド", String::from("-2"));
    } else if 65 <= atk && atk <= 84 {
        status.insert("db", String::from("-1"));
        status.insert("ビルド", String::from("-1"));
    } else if 85 <= atk && atk <= 124 {
        status.insert("db", String::from("0"));
        status.insert("ビルド", String::from("0"));
    } else if 125 <= atk && atk <= 164 {
        status.insert("db", String::from("+1d4"));
        status.insert("ビルド", String::from("1"));
    } else {
        status.insert("db", String::from("+1d6"));
        status.insert("ビルド", String::from("2"));
    }

    let dex = status["DEX"].parse::<usize>().unwrap();
    let siz = status["SIZ"].parse::<usize>().unwrap();
    let str = status["STR"].parse::<usize>().unwrap();
    if dex < siz && str > siz {
        status.insert("MOV", String::from("7"));
    } else if dex > siz && str > siz {
        status.insert("MOV", String::from("9"));
    } else {
        status.insert("MOV", String::from("8"));
    }
    let msg = format!(
        "`探索者作成`\n\
        STR: {}\n\
        CON: {}\n\
        SIZ: {}\n\
        DEX: {}\n\
        APP: {}\n\
        INT: {}\n\
        POW: {}\n\
        EDU: {}\n\
        SAN: {}\n\
        MP: {}\n\
        幸運: {}\n\
        耐久力: {}\n\
        db: {}\n\
        ビルド: {}\n\
        MOV: {}", 
        status["STR"], status["CON"], status["SIZ"], status["DEX"], status["APP"], status["INT"], 
        status["POW"], status["EDU"], status["SAN"], status["MP"], status["幸運"], status["耐久力"], 
        status["db"], status["ビルド"], status["MOV"]
    );
    msg
}

pub fn set_status_msg(skill_name: &str, before: usize, after: usize, operator: &str, corr: usize) -> String {
    let msg = format!(
        "`ステータス修正 / {}({}) {} {}`\n\
        => **{}**",
        skill_name, before, operator, corr, after
    );
    msg
}