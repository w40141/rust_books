fn main() {
    let msg = String::from("I am a student.");
    let g1 = msg;
    println!("{g1}");
    let ten = 10;
    let t = x2(&ten);
    println!("{t}");
    println!("{ten}");
    let banana = Item("Banana".to_string(), 300);
    let apple = Item("Apple".to_string(), 200);
    let orange = Item("Orange".to_string(), 150);
    let items = vec![banana, apple, orange];
    let total = print_add_sum_items(&items);
    println!("{total}");
    let s: String = String::from("abcdefghijk");
    let slice = &s[1..3];
    println!("{slice}");
    let li: Vec<i32> = (1..10).collect();
    let s = sum_slice(0, &li);
    println!("{s}")
}

fn sum_slice(start: i32, slice: &[i32]) -> i32 {
    match slice.first() {
        Some(v) => sum_slice(start + v, &slice[1..]),
        _ => start,
    }
}

fn x2(arg: &i32) -> i32 {
    arg * 2
}

struct Item(String, i64);

fn print_tuple(item: &Item) {
    println!("{}を{}円で購入", item.0, item.1);
}

fn print_add_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for it in items {
        print_tuple(&it);
        total += it.1;
    }
    total
}
