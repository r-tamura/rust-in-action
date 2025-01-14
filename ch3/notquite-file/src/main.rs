//! ファイル群を段階的にシミュレートする
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter,) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl File {
    /// 新規ファイルは空とみなすが、ファイル名は必須
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// ファイルの長さ（バイト数）を返す
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// ファイル名を返す
    pub fn name(&self) -> String {
        self.name.clone()
    }

    fn new_with_data(
        name: &str,
        data: &Vec<u8>,
    ) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp  = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter,) -> fmt::Result {
        write!(f, "<{}({})>", self.name, self.state)
    }
}


fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f_data: Vec<u8> = vec![114, 117, 115, 116, 23];
    let f = File::new("test.text");

    let mut buffer : Vec<u8> = vec![];

    if f.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    let f = open(f).unwrap();
    let f_length = f.read(&mut buffer).unwrap();
    let f = close(f).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f);
    println!("{}", f);
    println!("{} is {} bytes long", f.name, f_length);
    println!("{}", text)
}
