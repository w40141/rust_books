use crate::{easy_for, echo_num, map_init, out_html};

pub fn sec_01() {
    echo_num![20, 30, 40, 50];
    easy_for! {
        for i = 0 to 10 step 3 {
            println!("i={i}");
        }
    }
    let week = map_init!["mon" => "月曜", "tue" => "火曜", "wed" => "水曜", "thu" => "木曜", "fri" => "金曜", "sat" => "土曜", "sun" => "[選択]日曜"];
    println!("mon={}", week["mon"]);
    out_html!(
        html[
        head[title["test"]]
        body[
        h1["test"]
        p["This is test."]
    ]
    ]
    )
}
