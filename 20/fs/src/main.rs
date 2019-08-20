use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use std::os::unix;
use std::path::Path;

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("Hello, world!");

    fs::remove_dir_all("/tmp/a").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("mkdir /tmp/a");
    match fs::create_dir("/tmp/a") {
        Err(why) => println!("! {:?}, {:?}", why.kind(), why.description()),
        Ok(_) => {}
    }

    println!("`echo hello > /tmp/a/b.txt`");
    echo("hello", &Path::new("/tmp/a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`mkdir -p /tmp/a/c/d`");
    // 递归地创建一个目录，返回 `io::Result<()>`
    fs::create_dir_all("/tmp/a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch /tmp/a/c/e.txt`");
    touch(&Path::new("/tmp/a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ln -s /tmp/a/b.txt /tmp/a/c/b.txt`");
    // 创建一个符号链接，返回 `io::Result<()>`
    if cfg!(target_family = "unix") {
        unix::fs::symlink("/tmp/a/b.txt", "/tmp/a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    println!("`cat /tmp/a/c/b.txt`");
    match cat(&Path::new("/tmp/a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls /tmp/a`");
    match fs::read_dir("/tmp/a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }

    println!("`rmdir /tmp/a/c/d`");
    // 移除一个空目录，返回 `io::Result<()>`
    fs::remove_dir("/tmp/a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
