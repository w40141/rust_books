use anyhow::{Ok, Result};
// use regex::Regex;
use reqwest::blocking;
use scraper::{Html, Selector};

pub fn main() -> Result<()> {
    // let url = String::from("https://blog.rust-lang.org/");
    let url = String::from(
        // "https://www.db.yugioh-card.com/yugiohdb/card_search.action?ope=2&cid=4007&request_locale=ja",
        "https://www.db.yugioh-card.com/yugiohdb/card_search.action?ope=2&cid=14730&request_locale=ja",
        // "https://www.db.yugioh-card.com/yugiohdb/card_search.action?ope=2&cid=15109&request_locale=ja",
    );
    let body = blocking::get(url)?.text()?;
    let doc = Html::parse_document(&body);

    let sel_attr = Selector::parse("td div.item_box span.item_box_value").unwrap();
    let ele = doc.select(&sel_attr);
    for e in ele {
        println!("{}", e.inner_html().trim());
    }
    println!("");

    let sel_type = Selector::parse("td div.item_box.t_center").unwrap();
    let ele = doc.select(&sel_type);
    for e in ele {
        let fragment = Html::parse_fragment(&e.inner_html());
        let ee = fragment
            .root_element()
            .tree()
            .values()
            .into_iter()
            .last()
            .unwrap()
            .as_text()
            .unwrap()
            .trim();
        let types: Vec<&str> = ee.split("／").map(|t| t.trim()).collect();
        println!("{:?}", types);
        // let re = Regex::new("^*罠$").unwrap();
        // match re.find(ee) {
        //     Some(v) => {re.strip_suffix_of},
        //     _ => println!("No"),
        // }
    }
    println!("");

    let sel_name = Selector::parse("div h1").unwrap();
    let ele = doc.select(&sel_name);
    for e in ele {
        // println!("{:?}", e.inner_html().trim());
        let fragment = Html::parse_fragment(&e.inner_html());
        let ee = fragment.root_element().tree().values().into_iter();
        for i in ee {
            if i.is_text() {
                println!("{:?}", i.as_text().unwrap().trim());
            }
        }
    }
    println!("");

    let sel_text = Selector::parse("td div.item_box_text").unwrap();
    let ele = doc.select(&sel_text);
    for e in ele {
        let fragment = Html::parse_fragment(&e.inner_html());
        let ee = fragment
            .root_element()
            .tree()
            .values()
            .into_iter()
            .last()
            .unwrap()
            .as_text()
            .unwrap()
            .trim();
        println!("{ee}");
    }
    Ok(())
}
