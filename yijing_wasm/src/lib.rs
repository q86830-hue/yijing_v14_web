// Copyright 2024 易经V14推理引擎项目
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use wasm_bindgen::prelude::*;

static GUA_NAMES: [&str; 64] = [
    "乾","坤","屯","蒙","需","讼","师","比","小畜","履","泰","否","同人","大有","谦","豫","随","蛊","临","观","噬嗑","贲","剥","复","无妄","大畜","颐","大过","坎","离","咸","恒","遁","大壮","晋","明夷","家人","睽","蹇","解","损","益","夬","姤","萃","升","困","井","革","鼎","震","艮","渐","归妹","丰","旅","巽","兑","涣","节","中孚","小过","既济","未济"
];

static PALACE_NAMES: [&str; 8] = ["乾宫","兑宫","离宫","震宫","巽宫","坎宫","艮宫","坤宫"];
static YAO_NAMES: [&str; 6] = ["初爻","二爻","三爻","四爻","五爻","上爻"];
static WUXING: [&str; 8] = ["金","金","火","木","木","水","土","土"];
static SIX_SHEN: [&str; 6] = ["青龙","玄武","白虎","螣蛇","勾陈","朱雀"];

static BAGUA: [[u8; 3]; 8] = [
    [1,1,1],[1,1,0],[1,0,1],[1,0,0],[0,1,0],[0,1,1],[0,0,1],[0,0,0]
];

static SHI_YING: [[usize; 2]; 64] = [
    [5,3],[0,3],[1,4],[2,5],[3,0],[4,1],[5,2],[0,1],
    [5,3],[0,3],[1,4],[2,5],[3,0],[4,1],[5,2],[0,1],
    [5,3],[0,3],[1,4],[2,5],[3,0],[4,1],[5,2],[0,1],
    [5,3],[0,3],[1,4],[2,5],[3,0],[4,1],[5,2],[0,1],
    [5,3],[0,3],[1,4],[2,5],[3,0],[4,1],[5,2],[0,1],
    [5,3],[0,3],[1,4],[2,5],[3,0],[4,1],[5,2],[0,1],
    [5,3],[0,3],[1,4],[2,5],[3,0],[4,1],[5,2],[0,1],
    [5,3],[0,3],[1,4],[2,5],[3,0],[4,1],[5,2],[0,1]
];

fn hash_string(s: &str) -> usize {
    let mut h: usize = 0;
    for c in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(c as usize);
    }
    h
}

fn gua_to_liu(gua_idx: usize) -> [u8; 6] {
    let shang = BAGUA[gua_idx / 8];
    let xia = BAGUA[gua_idx % 8];
    [shang[0], shang[1], shang[2], xia[0], xia[1], xia[2]]
}

fn liu_to_gua(liu: &[u8; 6]) -> usize {
    let mut s: usize = 0;
    for i in 0..3 { s += (liu[i] as usize) << (2 - i); }
    let mut x: usize = 0;
    for i in 3..6 { x += (liu[i] as usize) << (5 - i); }
    s * 8 + x
}

fn change_gua(gua_idx: usize, pos: usize) -> usize {
    let mut liu = gua_to_liu(gua_idx);
    liu[pos] = 1 - liu[pos];
    liu_to_gua(&liu)
}

fn get_wuxing(gua_idx: usize) -> &'static str { WUXING[gua_idx % 8] }

fn get_shi_ying(gua_idx: usize) -> (usize, usize) {
    let r = SHI_YING[gua_idx];
    (r[0], r[1])
}

fn get_liu_qin(gua_idx: usize, yao_idx: usize) -> &'static str {
    let gua_wx = get_wuxing(gua_idx);
    let yao_wx = WUXING[(gua_idx / 8 + yao_idx) % 8];
    let (sheng, ke, body) = match gua_wx {
        "金" => ("水", "木", "土"),
        "水" => ("木", "火", "金"),
        "木" => ("火", "土", "水"),
        "火" => ("土", "金", "木"),
        "土" => ("金", "木", "火"),
        _ => ("水", "木", "土"),
    };
    if yao_wx == sheng { "子孙" }
    else if yao_wx == ke { "官鬼" }
    else if yao_wx == body { "妻财" }
    else if yao_wx == gua_wx { "兄弟" }
    else { "父母" }
}

fn judge(gua_idx: usize, new_gua_idx: usize) -> &'static str {
    let orig_wx = get_wuxing(gua_idx);
    let new_wx = get_wuxing(new_gua_idx);
    if orig_wx == new_wx { return "平"; }
    let (sheng, ke) = match orig_wx {
        "金" => ("水", "木"),
        "水" => ("木", "火"),
        "木" => ("火", "土"),
        "火" => ("土", "金"),
        "土" => ("金", "木"),
        _ => ("水", "木"),
    };
    if new_wx == sheng { "吉" }
    else if new_wx == ke { "凶" }
    else { "平" }
}

fn calc_gua(input: &str) -> usize {
    let h = hash_string(input);
    h.wrapping_rem(64)
}

#[wasm_bindgen]
pub fn divine(input: &str) -> String {
    let gua_idx = calc_gua(input);
    let yao_pos = gua_idx % 6;
    let new_gua_idx = change_gua(gua_idx, yao_pos);
    let (shi, ying) = get_shi_ying(gua_idx);
    let (new_shi, new_ying) = get_shi_ying(new_gua_idx);
    let auspicious = judge(gua_idx, new_gua_idx);

    let mut yao_list_json = String::from("[");
    for i in 0..6 {
        if i > 0 { yao_list_json.push(','); }
        let shi_ying_str = if shi == i { "世" } else if ying == i { "应" } else { "—" };
        yao_list_json.push_str(&format!(
            r#"{{"yao_name":"{}","shi_ying":"{}","liu_qin":"{}","liu_sheng":"{}","dong_jing":"{}"}}"#,
            YAO_NAMES[i], shi_ying_str, get_liu_qin(gua_idx, i), SIX_SHEN[i],
            if i == yao_pos { "动" } else { "静" }
        ));
    }
    yao_list_json.push(']');

    let mut new_yao_list_json = String::from("[");
    for i in 0..6 {
        if i > 0 { new_yao_list_json.push(','); }
        let shi_ying_str = if new_shi == i { "世" } else if new_ying == i { "应" } else { "—" };
        new_yao_list_json.push_str(&format!(
            r#"{{"yao_name":"{}","shi_ying":"{}","liu_qin":"{}","liu_sheng":"{}","dong_jing":"{}"}}"#,
            YAO_NAMES[i], shi_ying_str, get_liu_qin(new_gua_idx, i), SIX_SHEN[i],
            if i == yao_pos { "动" } else { "静" }
        ));
    }
    new_yao_list_json.push(']');

    let yao_text = match gua_idx {
        0 => ["潜龙勿用","见龙在田","君子终日乾乾","或跃在渊","飞龙在天","亢龙有悔"][yao_pos],
        1 => ["履霜坚冰至","直方大","含章可贞","括囊无咎","黄裳元吉","龙战于野"][yao_pos],
        _ => &format!("第{}卦第{}爻", gua_idx + 1, yao_pos + 1),
    };

    serde_json::json!({
        "gua_name": GUA_NAMES[gua_idx],
        "gua_index": gua_idx,
        "palace": PALACE_NAMES[gua_idx % 8],
        "wuxing": get_wuxing(gua_idx),
        "change_yao": yao_pos,
        "change_yao_name": format!("{}({}爻)", YAO_NAMES[yao_pos], yao_pos + 1),
        "new_gua": GUA_NAMES[new_gua_idx],
        "new_gua_index": new_gua_idx,
        "new_palace": PALACE_NAMES[new_gua_idx % 8],
        "new_wuxing": get_wuxing(new_gua_idx),
        "auspicious": auspicious,
        "yao_list": serde_json::from_str::<serde_json::Value>(&yao_list_json).unwrap(),
        "new_yao_list": serde_json::from_str::<serde_json::Value>(&new_yao_list_json).unwrap(),
        "yao_text": yao_text
    }).to_string()
}

#[wasm_bindgen]
pub fn get_gua_info(gua_idx: usize) -> String {
    let liu = gua_to_liu(gua_idx);
    serde_json::json!({
        "gua_name": GUA_NAMES[gua_idx],
        "gua_index": gua_idx,
        "palace": PALACE_NAMES[gua_idx % 8],
        "wuxing": get_wuxing(gua_idx),
        "liu": liu,
        "shi": SHI_YING[gua_idx][0],
        "ying": SHI_YING[gua_idx][1]
    }).to_string()
}
