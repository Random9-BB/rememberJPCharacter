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
        KanaItem { image: "/assets/あ.gif".into(), romaji: "a".into() },
        KanaItem { image: "/assets/い.gif".into(), romaji: "i".into() },
        KanaItem { image: "/assets/う.gif".into(), romaji: "u".into() },
        KanaItem { image: "/assets/え.gif".into(), romaji: "e".into() },
        KanaItem { image: "/assets/お.gif".into(), romaji: "o".into() },
        KanaItem { image: "/assets/か.gif".into(), romaji: "ka".into() },
        KanaItem { image: "/assets/き.gif".into(), romaji: "ki".into() },
        KanaItem { image: "/assets/く.gif".into(), romaji: "ku".into() },
        KanaItem { image: "/assets/け.gif".into(), romaji: "ke".into() },
        KanaItem { image: "/assets/こ.gif".into(), romaji: "ko".into() },
        KanaItem { image: "/assets/さ.gif".into(), romaji: "sa".into() },
        KanaItem { image: "/assets/し.gif".into(), romaji: "shi".into() },
        KanaItem { image: "/assets/す.gif".into(), romaji: "su".into() },
        KanaItem { image: "/assets/せ.gif".into(), romaji: "se".into() },
        KanaItem { image: "/assets/そ.gif".into(), romaji: "so".into() },
        KanaItem { image: "/assets/た.gif".into(), romaji: "ta".into() },
        KanaItem { image: "/assets/ち.gif".into(), romaji: "chi".into() },
        KanaItem { image: "/assets/つ.gif".into(), romaji: "tsu".into() },
        KanaItem { image: "/assets/て.gif".into(), romaji: "te".into() },
        KanaItem { image: "/assets/と.gif".into(), romaji: "to".into() },
        KanaItem { image: "/assets/な.gif".into(), romaji: "na".into() },
        KanaItem { image: "/assets/に.gif".into(), romaji: "ni".into() },
        KanaItem { image: "/assets/ぬ.gif".into(), romaji: "nu".into() },
        KanaItem { image: "/assets/ね.gif".into(), romaji: "ne".into() },
        KanaItem { image: "/assets/の.gif".into(), romaji: "no".into() },
        KanaItem { image: "/assets/は.gif".into(), romaji: "ha".into() },
        KanaItem { image: "/assets/ひ.gif".into(), romaji: "hi".into() },
        KanaItem { image: "/assets/ふ.gif".into(), romaji: "fu".into() },
        KanaItem { image: "/assets/へ.gif".into(), romaji: "he".into() },
        KanaItem { image: "/assets/ほ.gif".into(), romaji: "ho".into() },
        KanaItem { image: "/assets/ま.gif".into(), romaji: "ma".into() },
        KanaItem { image: "/assets/み.gif".into(), romaji: "mi".into() },
        KanaItem { image: "/assets/む.gif".into(), romaji: "mu".into() },
        KanaItem { image: "/assets/め.gif".into(), romaji: "me".into() },
        KanaItem { image: "/assets/も.gif".into(), romaji: "mo".into() },
        KanaItem { image: "/assets/や.gif".into(), romaji: "ya".into() },
        KanaItem { image: "/assets/ゆ.gif".into(), romaji: "yu".into() },
        KanaItem { image: "/assets/よ.gif".into(), romaji: "yo".into() },
        KanaItem { image: "/assets/ら.gif".into(), romaji: "ra".into() },
        KanaItem { image: "/assets/り.gif".into(), romaji: "ri".into() },
        KanaItem { image: "/assets/る.gif".into(), romaji: "ru".into() },
        KanaItem { image: "/assets/れ.gif".into(), romaji: "re".into() },
        KanaItem { image: "/assets/ろ.gif".into(), romaji: "ro".into() },
        KanaItem { image: "/assets/わ.gif".into(), romaji: "wa".into() },
        KanaItem { image: "/assets/を.gif".into(), romaji: "wo".into() },
        KanaItem { image: "/assets/ん.gif".into(), romaji: "n".into() },

        // Katakana
        KanaItem { image: "/assets/ア.gif".into(), romaji: "a".into() },
        KanaItem { image: "/assets/イ.gif".into(), romaji: "i".into() },
        KanaItem { image: "/assets/ウ.gif".into(), romaji: "u".into() },
        KanaItem { image: "/assets/エ.gif".into(), romaji: "e".into() },
        KanaItem { image: "/assets/オ.gif".into(), romaji: "o".into() },
        KanaItem { image: "/assets/カ.gif".into(), romaji: "ka".into() },
        KanaItem { image: "/assets/キ.gif".into(), romaji: "ki".into() },
        KanaItem { image: "/assets/ク.gif".into(), romaji: "ku".into() },
        KanaItem { image: "/assets/ケ.gif".into(), romaji: "ke".into() },
        KanaItem { image: "/assets/コ.gif".into(), romaji: "ko".into() },
        KanaItem { image: "/assets/サ.gif".into(), romaji: "sa".into() },
        KanaItem { image: "/assets/シ.gif".into(), romaji: "shi".into() },
        KanaItem { image: "/assets/ス.gif".into(), romaji: "su".into() },
        KanaItem { image: "/assets/セ.gif".into(), romaji: "se".into() },
        KanaItem { image: "/assets/ソ.gif".into(), romaji: "so".into() },
        KanaItem { image: "/assets/タ.gif".into(), romaji: "ta".into() },
        KanaItem { image: "/assets/チ.gif".into(), romaji: "chi".into() },
        KanaItem { image: "/assets/ツ.gif".into(), romaji: "tsu".into() },
        KanaItem { image: "/assets/テ.gif".into(), romaji: "te".into() },
        KanaItem { image: "/assets/ト.gif".into(), romaji: "to".into() },
        KanaItem { image: "/assets/ナ.gif".into(), romaji: "na".into() },
        KanaItem { image: "/assets/ニ.gif".into(), romaji: "ni".into() },
        KanaItem { image: "/assets/ヌ.gif".into(), romaji: "nu".into() },
        KanaItem { image: "/assets/ネ.gif".into(), romaji: "ne".into() },
        KanaItem { image: "/assets/ノ.gif".into(), romaji: "no".into() },
        KanaItem { image: "/assets/ハ.gif".into(), romaji: "ha".into() },
        KanaItem { image: "/assets/ヒ.gif".into(), romaji: "hi".into() },
        KanaItem { image: "/assets/フ.gif".into(), romaji: "fu".into() },
        KanaItem { image: "/assets/ヘ.gif".into(), romaji: "he".into() },
        KanaItem { image: "/assets/ホ.gif".into(), romaji: "ho".into() },
        KanaItem { image: "/assets/マ.gif".into(), romaji: "ma".into() },
        KanaItem { image: "/assets/ミ.gif".into(), romaji: "mi".into() },
        KanaItem { image: "/assets/ム.gif".into(), romaji: "mu".into() },
        KanaItem { image: "/assets/メ.gif".into(), romaji: "me".into() },
        KanaItem { image: "/assets/モ.gif".into(), romaji: "mo".into() },
        KanaItem { image: "/assets/ヤ.gif".into(), romaji: "ya".into() },
        KanaItem { image: "/assets/ユ.gif".into(), romaji: "yu".into() },
        KanaItem { image: "/assets/ヨ.gif".into(), romaji: "yo".into() },
        KanaItem { image: "/assets/ラ.gif".into(), romaji: "ra".into() },
        KanaItem { image: "/assets/リ.gif".into(), romaji: "ri".into() },
        KanaItem { image: "/assets/ル.gif".into(), romaji: "ru".into() },
        KanaItem { image: "/assets/レ.gif".into(), romaji: "re".into() },
        KanaItem { image: "/assets/ロ.gif".into(), romaji: "ro".into() },
        KanaItem { image: "/assets/ワ.gif".into(), romaji: "wa".into() },
        KanaItem { image: "/assets/ヲ.gif".into(), romaji: "wo".into() },
        KanaItem { image: "/assets/ン.gif".into(), romaji: "n".into() },
    ]
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_kana_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
