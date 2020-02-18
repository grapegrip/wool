use wool::{github_markdown_to_html};
use std::include_str;


fn main() {
   let test = include_str!("readme.md");
   println!("{}", github_markdown_to_html(test.into(), "readme.md".into()));
    
}
