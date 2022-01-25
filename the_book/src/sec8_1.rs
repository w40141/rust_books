pub fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);
    // let v2 = vec![1, 2, 3];
    // let second: &i32 = &v2[1];
    // let third: Option<&i32> = v2.get(2);
    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        *i += 50;
    }

    enum SpredsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let rows = vec![
        SpredsheetCell::Int(3),
        SpredsheetCell::Float(12.2),
        SpredsheetCell::Text(String::from("blue")),
    ];
}
