#![allow(unused_variables)]

use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

/// fileは、アクセス可能なファイルを意味する。
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}({})>", self.name, self.state)
    }
}

impl File {
    /// 新規ファイルは空とみなすが、ファイル名は必須
    ///
    /// # 例
    ///
    /// ```
    /// let f = File::new("f1.txt");
    /// ```
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// ファイルの長さ(バイト数)を返す
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// ファイル名を返す
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub fn main() {
    let f6 = File::new("f6.txt");
    println!("{:?}", f6);
    println!("{}", f6);
}
