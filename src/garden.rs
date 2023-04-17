mod fruits;
pub mod vegatables;

pub fn pluck_vegtable() {
    corrupt_vegtable();
    self::fruits::ripen();
}

fn corrupt_vegtable() {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("Default summary implementation");
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        todo!()
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // note that we dont need to explicitly specify lifetime of return type
    // its assumed to be same as that of &self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// can also do as
// pub fn notify<T: Summary>(item: &Summary) {}
pub fn notify(item: &impl Summary) {
    let my_str : &'static str = "Breaking news!";
    println!("{} {}", my_str, item.summarize());
    let ia = ImportantExcerpt { part:"hello"};
    let _b = ia.announce_and_return_part("some str");
}

