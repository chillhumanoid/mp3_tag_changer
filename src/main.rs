use id3::{Tag, Version};

fn main() {
    let mut tag = Tag::read_from_path("/home/jthorne/Music/Heavy Heart.mp3").unwrap();
    println!("{}", tag.artist().unwrap());
}
