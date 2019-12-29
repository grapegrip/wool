use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
use std::fs::File;
use std::io::Write;
use rouille::Response;

mod template;
mod prism;
mod cli;


fn main() {
    
    let matches = cli::get_cli_matches();
    let contents = fs::read_to_string(matches.value_of("infile").unwrap()).unwrap();
    let options = set_opts();
    let markdown = markdown_to_html(&contents, &options);
    let highlight = matches.is_present("highlight");

    if matches.is_present("export-flag") {
        let infile = matches.value_of("infile").unwrap(); 
       
        if matches.is_present("outfile") {
            let outfile: &str = matches.value_of("outfile").unwrap();
            let mut file = File::create(outfile).unwrap();
            let outfile_prefix: Vec<&str> = matches.value_of("outfile").unwrap().split(".").collect();
            let boilerplate_name = format!("{}.md", outfile_prefix[0]); 
            
            let boilerplate: &str = &template::format_boilerplate(&boilerplate_name);
            let css: &str = template::CSS;
            let footer: &str = template::FOOTER;

            let prism: &str = prism::PRISM;
            let prismcss: &str = prism::CSS;
           
            file.write_all(boilerplate.as_bytes()).expect("could not write file");
            if highlight {file.write_all(prismcss.as_bytes()).expect("could not write file");};
            file.write_all(css.as_bytes()).expect("could not write file");
            file.write_all(markdown.as_bytes()).expect("could not write file");
            if highlight {file.write_all(prism.as_bytes()).expect("could not write file");};
            file.write_all(footer.as_bytes()).expect("could not write file");
        }
        
        else { 
            let outfile = &infile.replace("md", "html");
            let mut file = File::create(outfile).unwrap();
            let boilerplate: &str = &template::format_boilerplate(&infile); 
            let css: &str = template::CSS;
            let footer: &str = template::FOOTER;
                
            let prism: &str = prism::PRISM;
            let prismcss: &str = prism::CSS;
        
        
            file.write_all(boilerplate.as_bytes()).expect("could not write file");
            if highlight {file.write_all(prismcss.as_bytes()).expect("could not highlight file");};
            file.write_all(css.as_bytes()).expect("could not read file");
            file.write_all(markdown.as_bytes()).expect("could not read file");
            if highlight {file.write_all(prism.as_bytes()).expect("could not highlight file");};
            file.write_all(footer.as_bytes()).expect("could not write file");
        }
    } 
    
    else {
        println!("listening on http://localhost:10009");
        
        rouille::start_server("localhost:10009", move |request| {
        {
            let response = rouille::match_assets(&request, ".");

            if response.is_success() {
                return response;
            }
        }
        
          let mut contents = String::new();
          let boilerplate: &str = &template::format_boilerplate(matches.value_of("infile").unwrap());
          let css: &str = template::CSS;
          let footer: &str = template::FOOTER;
          let prism: &str = prism::PRISM;
          let prismcss: &str = prism::CSS;
          contents.push_str(boilerplate);
          if highlight { contents.push_str(prismcss);};
          contents.push_str(css);
          contents.push_str(&markdown);
          if highlight { contents.push_str(prism);};
          contents.push_str(footer);

          
          Response::html(contents)
          
      });
        
    }
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
