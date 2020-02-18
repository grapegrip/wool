use comrak::{markdown_to_html, ComrakOptions};
mod template;

pub fn github_markdown_to_html(md: String, filename: String) -> String {
    let mut contents = String::from("");
    let options = set_opts();
    let markdown = markdown_to_html(&md, &options);
    let boilerplate = template::format_boilerplate(&filename);
    let css: &str = template::CSS;
    let footer: &str = template::FOOTER;
    contents.push_str(boilerplate.as_str());
    contents.push_str(css);
    contents.push_str(&markdown);
    contents.push_str(footer);
    contents    
}

fn set_opts() -> ComrakOptions {
    let mut options = ComrakOptions::default();
    options.unsafe_ = true;
    options.github_pre_lang = true;
    options.ext_table = true;
    options.ext_tagfilter = true;
    options.ext_strikethrough = true;
    options.ext_footnotes = true;
    options.ext_superscript = true;
    options.ext_autolink = true;
    options.ext_tasklist = true;
    options.ext_description_lists = true;
    options
}
