pub mod url;

use crate::url::download_image;
use tokio::time;

#[tokio::main]
async fn main() {
    // for i in 1..=3 {
    //     println!("#{i}を開始");
    //     let s = read_longtime().await;
    //     println!("{s}");
    //     let s = async {
    //         time::sleep(time::Duration::from_secs(1)).await;
    //         String::from("長い読み込み完了(block)")
    //     }
    //     .await;
    //     println!("{s}");
    // }
    // let f = say_later("諦めるのに時がある。");
    // println!("捜すのに時がある。");
    // f.await;
    // tokio::spawn(say_later(3, "毎日が宴会である。"));
    // tokio::spawn(say_later(2, "陽気な人の心には。。。"));
    // tokio::spawn(say_later(1, "苦しむ人にはどの日も悪い日で。。。"));
    // time::sleep(time::Duration::from_secs(4)).await;
    // println!("-----");
    //
    // tokio::join!(
    //     say_later(2, "一生懸命働く充実感。。。"),
    //     say_later(3, "人にとってこれ以上の幸せはない。"),
    //     say_later(1, "食べ、飲み"),
    // );
    for title in ["温泉", "書道"] {
        download_image(title).await;
    }
}

// async fn say_later(msg: &'static str) {
//     println!("{msg}");
// }

async fn say_later(sec: u64, msg: &str) {
    time::sleep(time::Duration::from_secs(sec)).await;
    println!("{sec}: {msg}");
}
async fn read_longtime() -> String {
    time::sleep(time::Duration::from_secs(1)).await;
    String::from("長い読み込み完了(fn)")
}
