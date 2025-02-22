use tauri::command;


#[derive(serde::Serialize)]
struct KanaItem {
    image: String,
    romaji: String,
}

#[command]
fn get_kana_list() -> Vec<KanaItem> {
    vec![
        // Hiragana
        KanaItem { image: "../../src/assets/あ.gif".into(), romaji: "a".into() },
        KanaItem { image: "../../src/assets/い.gif".into(), romaji: "i".into() },
        KanaItem { image: "../../src/assets/う.gif".into(), romaji: "u".into() },
        KanaItem { image: "../../src/assets/え.gif".into(), romaji: "e".into() },
        KanaItem { image: "../../src/assets/お.gif".into(), romaji: "o".into() },
        KanaItem { image: "../../src/assets/か.gif".into(), romaji: "ka".into() },
        KanaItem { image: "../../src/assets/き.gif".into(), romaji: "ki".into() },
        KanaItem { image: "../../src/assets/く.gif".into(), romaji: "ku".into() },
        KanaItem { image: "../../src/assets/け.gif".into(), romaji: "ke".into() },
        KanaItem { image: "../../src/assets/こ.gif".into(), romaji: "ko".into() },
        KanaItem { image: "../../src/assets/さ.gif".into(), romaji: "sa".into() },
        KanaItem { image: "../../src/assets/し.gif".into(), romaji: "shi".into() },
        KanaItem { image: "../../src/assets/す.gif".into(), romaji: "su".into() },
        KanaItem { image: "../../src/assets/せ.gif".into(), romaji: "se".into() },
        KanaItem { image: "../../src/assets/そ.gif".into(), romaji: "so".into() },
        KanaItem { image: "../../src/assets/た.gif".into(), romaji: "ta".into() },
        KanaItem { image: "../../src/assets/ち.gif".into(), romaji: "chi".into() },
        KanaItem { image: "../../src/assets/つ.gif".into(), romaji: "tsu".into() },
        KanaItem { image: "../../src/assets/て.gif".into(), romaji: "te".into() },
        KanaItem { image: "../../src/assets/と.gif".into(), romaji: "to".into() },
        KanaItem { image: "../../src/assets/な.gif".into(), romaji: "na".into() },
        KanaItem { image: "../../src/assets/に.gif".into(), romaji: "ni".into() },
        KanaItem { image: "../../src/assets/ぬ.gif".into(), romaji: "nu".into() },
        KanaItem { image: "../../src/assets/ね.gif".into(), romaji: "ne".into() },
        KanaItem { image: "../../src/assets/の.gif".into(), romaji: "no".into() },
        KanaItem { image: "../../src/assets/は.gif".into(), romaji: "ha".into() },
        KanaItem { image: "../../src/assets/ひ.gif".into(), romaji: "hi".into() },
        KanaItem { image: "../../src/assets/ふ.gif".into(), romaji: "fu".into() },
        KanaItem { image: "../../src/assets/へ.gif".into(), romaji: "he".into() },
        KanaItem { image: "../../src/assets/ほ.gif".into(), romaji: "ho".into() },
        KanaItem { image: "../../src/assets/ま.gif".into(), romaji: "ma".into() },
        KanaItem { image: "../../src/assets/み.gif".into(), romaji: "mi".into() },
        KanaItem { image: "../../src/assets/む.gif".into(), romaji: "mu".into() },
        KanaItem { image: "../../src/assets/め.gif".into(), romaji: "me".into() },
        KanaItem { image: "../../src/assets/も.gif".into(), romaji: "mo".into() },
        KanaItem { image: "../../src/assets/や.gif".into(), romaji: "ya".into() },
        KanaItem { image: "../../src/assets/ゆ.gif".into(), romaji: "yu".into() },
        KanaItem { image: "../../src/assets/よ.gif".into(), romaji: "yo".into() },
        KanaItem { image: "../../src/assets/ら.gif".into(), romaji: "ra".into() },
        KanaItem { image: "../../src/assets/り.gif".into(), romaji: "ri".into() },
        KanaItem { image: "../../src/assets/る.gif".into(), romaji: "ru".into() },
        KanaItem { image: "../../src/assets/れ.gif".into(), romaji: "re".into() },
        KanaItem { image: "../../src/assets/ろ.gif".into(), romaji: "ro".into() },
        KanaItem { image: "../../src/assets/わ.gif".into(), romaji: "wa".into() },
        KanaItem { image: "../../src/assets/を.gif".into(), romaji: "wo".into() },
        KanaItem { image: "../../src/assets/ん.gif".into(), romaji: "n".into() },

        // Katakana
        KanaItem { image: "../../src/assets/ア.gif".into(), romaji: "a".into() },
        KanaItem { image: "../../src/assets/イ.gif".into(), romaji: "i".into() },
        KanaItem { image: "../../src/assets/ウ.gif".into(), romaji: "u".into() },
        KanaItem { image: "../../src/assets/エ.gif".into(), romaji: "e".into() },
        KanaItem { image: "../../src/assets/オ.gif".into(), romaji: "o".into() },
        KanaItem { image: "../../src/assets/カ.gif".into(), romaji: "ka".into() },
        KanaItem { image: "../../src/assets/キ.gif".into(), romaji: "ki".into() },
        KanaItem { image: "../../src/assets/ク.gif".into(), romaji: "ku".into() },
        KanaItem { image: "../../src/assets/ケ.gif".into(), romaji: "ke".into() },
        KanaItem { image: "../../src/assets/コ.gif".into(), romaji: "ko".into() },
        KanaItem { image: "../../src/assets/サ.gif".into(), romaji: "sa".into() },
        KanaItem { image: "../../src/assets/シ.gif".into(), romaji: "shi".into() },
        KanaItem { image: "../../src/assets/ス.gif".into(), romaji: "su".into() },
        KanaItem { image: "../../src/assets/セ.gif".into(), romaji: "se".into() },
        KanaItem { image: "../../src/assets/ソ.gif".into(), romaji: "so".into() },
        KanaItem { image: "../../src/assets/タ.gif".into(), romaji: "ta".into() },
        KanaItem { image: "../../src/assets/チ.gif".into(), romaji: "chi".into() },
        KanaItem { image: "../../src/assets/ツ.gif".into(), romaji: "tsu".into() },
        KanaItem { image: "../../src/assets/テ.gif".into(), romaji: "te".into() },
        KanaItem { image: "../../src/assets/ト.gif".into(), romaji: "to".into() },
        KanaItem { image: "../../src/assets/ナ.gif".into(), romaji: "na".into() },
        KanaItem { image: "../../src/assets/ニ.gif".into(), romaji: "ni".into() },
        KanaItem { image: "../../src/assets/ヌ.gif".into(), romaji: "nu".into() },
        KanaItem { image: "../../src/assets/ネ.gif".into(), romaji: "ne".into() },
        KanaItem { image: "../../src/assets/ノ.gif".into(), romaji: "no".into() },
        KanaItem { image: "../../src/assets/ハ.gif".into(), romaji: "ha".into() },
        KanaItem { image: "../../src/assets/ヒ.gif".into(), romaji: "hi".into() },
        KanaItem { image: "../../src/assets/フ.gif".into(), romaji: "fu".into() },
        KanaItem { image: "../../src/assets/ヘ.gif".into(), romaji: "he".into() },
        KanaItem { image: "../../src/assets/ホ.gif".into(), romaji: "ho".into() },
        KanaItem { image: "../../src/assets/マ.gif".into(), romaji: "ma".into() },
        KanaItem { image: "../../src/assets/ミ.gif".into(), romaji: "mi".into() },
        KanaItem { image: "../../src/assets/ム.gif".into(), romaji: "mu".into() },
        KanaItem { image: "../../src/assets/メ.gif".into(), romaji: "me".into() },
        KanaItem { image: "../../src/assets/モ.gif".into(), romaji: "mo".into() },
        KanaItem { image: "../../src/assets/ヤ.gif".into(), romaji: "ya".into() },
        KanaItem { image: "../../src/assets/ユ.gif".into(), romaji: "yu".into() },
        KanaItem { image: "../../src/assets/ヨ.gif".into(), romaji: "yo".into() },
        KanaItem { image: "../../src/assets/ラ.gif".into(), romaji: "ra".into() },
        KanaItem { image: "../../src/assets/リ.gif".into(), romaji: "ri".into() },
        KanaItem { image: "../../src/assets/ル.gif".into(), romaji: "ru".into() },
        KanaItem { image: "../../src/assets/レ.gif".into(), romaji: "re".into() },
        KanaItem { image: "../../src/assets/ロ.gif".into(), romaji: "ro".into() },
        KanaItem { image: "../../src/assets/ワ.gif".into(), romaji: "wa".into() },
        KanaItem { image: "../../src/assets/ヲ.gif".into(), romaji: "wo".into() },
        KanaItem { image: "../../src/assets/ン.gif".into(), romaji: "n".into() },
    ]
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_kana_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
