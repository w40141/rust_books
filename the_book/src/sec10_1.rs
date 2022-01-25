fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'a', 'b', 'i'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // let integer_point = Point { x: 5, y: 10 };
    // let float_point = Point { x: 1.0, y: 5.0 };
    // println!("p.x = {}", integer_point.x());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest_number = list[0];
    for &item in list.iter() {
        if item > largest_number {
            largest_number = item;
        }
    }
    largest_number
}

// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest_number = list[0];
//     for &item in list.iter() {
//         if item > largest_number {
//             largest_number = item;
//         }
//     }
//     largest_number
// }

// fn largest_char(list: &[char]) -> char {
//     let mut largest_number = list[0];
//     for &item in list.iter() {
//         if item > largest_number {
//             largest_number = item;
//         }
//     }
//     largest_number
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }

    // fn distance_from_origin(&self) -> f32 {
    //     (self.x.pow(2) + self.y.pow(2)).sqrt()
    // }
}
