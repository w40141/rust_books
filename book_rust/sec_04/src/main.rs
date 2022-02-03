fn main() {
    let box1 = JewelyBox {
        price: 30,
        key_no: 1,
    };
    let box2 = TrapBox { damage: 10 };
    let box3 = JewelyBox {
        price: 20,
        key_no: 2,
    };

    let key_no = 2;
    open_box(&box1, key_no);
    open_box(&box2, key_no);
    open_box(&box3, key_no);
}

#[derive(Debug, PartialEq)]
enum BmiLevel {
    Lv0,
    Lv1,
    Lv2,
    Lv3,
    Lv4,
    Lv5,
}

#[derive(Debug)]
struct Body {
    height: f64,
    width: f64,
}

impl Body {
    fn new(height: f64, width: f64) -> Self {
        Self { height, width }
    }
    fn calc_bmi(&self) -> f64 {
        let h = self.height / 100.0;
        self.width / h.powf(2.0)
    }

    fn calc_per(&self) -> f64 {
        self.calc_bmi() / 22.0 * 100.0
    }

    fn calc_level(&self) -> BmiLevel {
        let bmi = self.calc_bmi();
        if bmi < 18.5 {
            BmiLevel::Lv0
        } else if bmi < 25.0 {
            BmiLevel::Lv1
        } else if bmi < 30.0 {
            BmiLevel::Lv2
        } else if bmi < 35.0 {
            BmiLevel::Lv3
        } else if bmi < 40.0 {
            BmiLevel::Lv4
        } else {
            BmiLevel::Lv5
        }
    }
}

struct Person {
    name: String,
    age: i32,
    body: Body,
}

impl Person {
    pub fn new(name: String, age: i32, height: f64, width: f64) -> Self {
        let body = Body::new(height, width);
        Self { name, age, body }
    }
}

trait TreasureBox {
    fn open(&self, key_no: i32) -> bool {
        self.get_key_no() == key_no
    }
    fn check(&self);
    fn get_key_no(&self) -> i32;
}

struct JewelyBox {
    price: i32,
    key_no: i32,
}

impl TreasureBox for JewelyBox {
    // fn open(&self, key_no: i32) -> bool {
    //     self.key_no == key_no
    // }

    fn check(&self) {
        println!("宝石箱だった！金貨{}枚を入手した。", self.price)
    }

    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

struct TrapBox {
    damage: i32,
}

impl TreasureBox for TrapBox {
    fn open(&self, _key: i32) -> bool {
        true
    }

    fn check(&self) {
        println!("罠だった！{}のダメージ。", self.damage)
    }

    fn get_key_no(&self) -> i32 {
        0
    }
}

struct EmptyBox {
    key_no: i32,
}

impl TreasureBox for EmptyBox {
    fn check(&self) {
        println!("空箱だった。")
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("鍵が合わず宝箱が開きません。");
        return;
    }
    tbox.check();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_test() {
        let taro = Body {
            height: 160.0,
            width: 70.0,
        };
        assert_eq!(160.0, taro.height);
        assert_eq!(70.0, taro.width);
    }

    #[test]
    fn person_test() {
        let age = 10;
        let height = 160.0;
        let width = 70.0;
        let bob = Person::new("bobby".to_string(), age, height, width);
        assert_eq!("bobby", bob.name);
        assert_eq!(10, bob.age);
        assert_eq!(BmiLevel::Lv2, bob.body.calc_level());
        let jiro = Person {
            name: String::from("jiro"),
            ..bob
        };
        assert_eq!("jiro", jiro.name);
        assert_eq!(10, jiro.age);
        assert_eq!(BmiLevel::Lv2, jiro.body.calc_level());
    }
}
