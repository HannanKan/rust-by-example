use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // 创建指向所需的文件的路径
    let path = Path::new("resource/hello.txt");
    let display = path.display();

    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(why) => panic!("couldn't open {}: {:?}", display, why),
        Ok(file) => file,
    };

    // 读取文件内容到一个字符串，返回 `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {:?}", display, why),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }

    use std::fs;
    // ./ 目录指的是 rust cargo 项目的更目录
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }

    // `file` 离开作用域，并且 `hello.txt` 文件将被关闭。
}
