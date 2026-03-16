use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably know, people",
        ),
        reply: false,
        repost: false,
    };

    notify(&post);
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}