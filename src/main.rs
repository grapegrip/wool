use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
use std::fs::File as Sync_File;
use std::io::Write;
use std::io::Error;
use url::Url;
use log::{info, debug, error};
use std::path::Path;
use hyper::header;

static INTERNAL_SERVER_ERROR_TEXT: &[u8] = b"Internal Server Error";
static METHOD_NOT_ALLOWED_TEXT: &[u8] = b"Method Not Allowed";
static NOT_FOUND_TEXT: &[u8] = b"Not Found la la";

use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper_staticfile::*;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use tokio_fs::File;
// use tokio_io::AsyncReadExt;
use tokio_sync::oneshot::{self, Sender};
use tokio_sync::*;
use tokio::*;
use tokio_fs::*;
use tokio_io::*;
use tokio_sync::*;
use tokio::io::AsyncReadExt;

mod cli;
mod prism;
mod template;
mod openurl;
mod katex;

type SenderListPtr = Arc<Mutex<Vec<Sender<()>>>>;

async fn update(updaters: SenderListPtr) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::builder();
    /*
    response.header("Cache-Control", "no-cache, no-store, must-revalidate");
    response.header("Pragma", "no-cache");
    response.header("Expires", "0");
    */

    let (tx, rx) = oneshot::channel();
    if let Ok(mut updaters) = updaters.lock() {
        updaters.push(tx);
    } else {
        error!("Internal error: mutex");
    }

    let _ = rx.await;
    Ok(response
        .body(Body::from("yes"))
        .expect("invalid response builder"))
}

async fn md_file() -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::builder();
    // response.header("Content-type", "text/html");
    
    let matches = cli::get_cli_matches();
    let contents = fs::read_to_string(matches.value_of("infile").unwrap()).unwrap();
    let mut options = set_opts();
    let latex = matches.is_present("katex");
    let highlight = matches.is_present("highlight");
    if latex { options.ext_superscript = false};
    let markdown = markdown_to_html(&contents, &options);

    let mut contents = String::new();

   let infile = matches.value_of("infile").unwrap();
   let boilerplate = template::format_boilerplate(infile);
   let css: &str = template::CSS;
   let footer: &str = template::FOOTER;
       let prism: &str = prism::PRISM;
       let prismcss: &str = prism::CSS; 
    
    
    // push it all into a container
    let reload_script: &str = 
        r#"
            <script type="text/javascript">
            function reload_check () {{
                var xhr = new XMLHttpRequest();
                xhr.overrideMimeType("text/plain");
                xhr.timeout = 100009;
                xhr.onreadystatechange = function () {{
                    if(this.readyState === 4) {{
                        if (this.status === 200) {{
                            if (this.responseText == "yes") {{
                                location.reload();
                            }} else {{
                                reload_check();
                            }}
                        }}
                    }}
                }}
                xhr.ontimeout = function () {{
                    reload_check();
                }}
                xhr.open("GET", "/update", true);
                xhr.send();
            }}
            reload_check();
            </script>
        "#;
            
    contents.push_str(boilerplate.as_str());
    if highlight {
        contents.push_str(prismcss);
    }
    contents.push_str(css);
    contents.push_str(&markdown);
    if highlight {
        contents.push_str(prism);
    }
    contents.push_str(reload_script);
    contents.push_str(footer);
    if latex {
        contents.push_str(katex::KATEX_CSS);
        contents.push_str(katex::KATEXJS);
        contents.push_str(katex::AUTO_RELOAD);
        contents.push_str(r#"<script> 
        renderMathInElement(document.body)
        </script> "#);
    }
    
    
    Ok(response.body(Body::from(contents)).expect("invalid response builder"))
}

// Will only serve files relative to the md file
async fn static_file(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::builder();
    let cwd = std::env::current_dir().expect("no working dir");
    if req.uri().path().len() > 1 {
        let mut fullpath = cwd.clone();
        // path() contains preceeding forward slash: /some/web/page
                    println!("{:?}, {:?}", fullpath, &req.uri());
        fullpath.push(&req.uri().path()[1..]);
        // canonicalize returns Err if path does not exist.
        if let Ok(fullpath) = fullpath.canonicalize() {
            if fullpath.starts_with(&cwd) {
                if let Ok(mut file) = File::open(&fullpath).await {
                    let mut buf = String::new();
                    //if file.read_to_string(&mut buf).await.is_ok() {
                        return Ok(response
                            .body(Body::from(buf))
                            .expect("invalid response builder"));
                    //}
                }
            }
        }
    }
   
    info!("{} not found", req.uri());
    // not found  
    //return Ok(Response::default())
   // return Ok(response.body(Body::from(req.uri())).expect("invalid response builder"))
        
    return Ok(response
        .body(Body::from("foo"))
        .expect("invalid response builder"))
        
}

async fn router(updaters: SenderListPtr, req: Request<Body> ) -> Result<Response<Body>, hyper::error::Error> {
    match req.uri().path() {
        "/update" => update(updaters).await,
        "/" => md_file().await,
        _ => {
          

            let root = Path::new("");
             let result = hyper_staticfile::resolve_path(&root, &req.uri().path()).await.unwrap();
             
             match result {
        ResolveResult::MethodNotMatched => return Ok(method_not_allowed()),
        ResolveResult::NotFound | ResolveResult::UriNotMatched => {
            return Ok(not_found(Path::new(&root.join("404.html"))).await)
        }
        _ => (),
    };
             
                Ok(hyper_staticfile::ResponseBuilder::new().request(&req).build(result).unwrap())

        }
    }
}
fn internal_server_error() -> Response<Body> {
    Response::builder()
        .header(header::CONTENT_TYPE, "text/plain")
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(INTERNAL_SERVER_ERROR_TEXT.into())
        .expect("Could not build Internal Server Error response")
}

async fn not_found(page_path: &Path) -> Response<Body> {
    if let Ok(mut file) = tokio::fs::File::open(page_path).await {
        let mut buf = Vec::new();
        if file.read_to_end(&mut buf).await.is_ok() {
            return Response::builder()
                .header(header::CONTENT_TYPE, "text/html")
                .status(StatusCode::NOT_FOUND)
                .body(buf.into())
                .expect("Could not build Not Found response");
        }

        return internal_server_error();
    }

    // Use a plain text response when page_path isn't available
    Response::builder()
        .header(header::CONTENT_TYPE, "text/plain")
        .status(StatusCode::NOT_FOUND)
        .body(NOT_FOUND_TEXT.into())
        .expect("Could not build Not Found response")
}

fn method_not_allowed() -> Response<Body> {
    Response::builder()
        .header(header::CONTENT_TYPE, "text/plain")
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(METHOD_NOT_ALLOWED_TEXT.into())
        .expect("Could not build Method Not Allowed response")
}

// attribution https://github.com/razorheadfx/grup
fn spawn_watcher(updaters: SenderListPtr) -> notify::Result<RecommendedWatcher> {
    
    let matches = cli::get_cli_matches();

    // this uses os specific file watching where possible (i.e. inotify on linux)
    // it forks of a mio event loop in the background and then calls the provided closure
    // with the yielded events
    let cwd = env::current_dir().unwrap(); 
    let md_file_name = cwd.join(&(matches.value_of("infile").unwrap())).to_owned(); 
    let mut file_event_watcher: RecommendedWatcher =
        RecommendedWatcher::new(move |event: notify::Result<Event>| {
            let event = match event {
                Ok(ev) => ev,
                Err(e) => {
                    println!("received file notifier error {:?}. Ignoring file event.", e);
                    return;
                }
            };
   
            match event.kind {
                EventKind::Create(_) => debug!("files created {:?}", &event.paths),
                EventKind::Modify(_) => debug!("files modified {:?}", &event.paths),
                _ => return,
            };

            if event.paths.iter().any(|path| path.eq(&md_file_name)) {
                if let Ok(mut updaters) = updaters.lock() {
                    for tx in updaters.drain(..) {
                        let _ = tx.send(());
                    }
                } else {
                    error!("Internal error: mutex");
                }
            }
        })?;

    file_event_watcher.watch(&cwd, RecursiveMode::NonRecursive)?;
    Ok(file_event_watcher)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let matches = cli::get_cli_matches();
    let contents = fs::read_to_string(matches.value_of("infile").unwrap()).unwrap();
    let options = set_opts();
    let markdown = markdown_to_html(&contents, &options);
    let highlight = matches.is_present("highlight");
    let browser = matches.is_present("browser");
    let latex = matches.is_present("katex");

    if matches.is_present("export-flag") {
        let infile = matches.value_of("infile").unwrap();
        if matches.is_present("outfile") {
            let outfile: &str = matches.value_of("outfile").unwrap();
            let mut file = Sync_File::create(outfile).unwrap();
            let outfile_prefix: Vec<&str> =
                matches.value_of("outfile").unwrap().split(".").collect();
            let boilerplate_name = format!("{}.md", outfile_prefix[0]);
            let infile = matches.value_of("infile").unwrap();
            let boilerplate = if matches.is_present("no-preview-frame") {
                template::format_boilerplate_no_preview(infile)
            } else {
                template::format_boilerplate(&boilerplate_name)
            };
            
            let css: &str = template::CSS;
            let footer: &str = template::FOOTER;

            let prism: &str = prism::PRISM;
            let prismcss: &str = prism::CSS;

            file.write_all(boilerplate.as_bytes())
                .expect("could not write file");
            if highlight {
                file.write_all(prismcss.as_bytes())
                    .expect("could not write file");
            };
            file.write_all(css.as_bytes())
                .expect("could not write file");
            file.write_all(markdown.as_bytes())
                .expect("could not write file");
            if highlight {
                file.write_all(prism.as_bytes())
                    .expect("could not write file");
            };
            file.write_all(footer.as_bytes())
                .expect("could not write file");
        } else {
            let outfile = &infile.replace("md", "html");
            let mut file = Sync_File::create(outfile).unwrap();
            
            let infile = matches.value_of("infile").unwrap();
            let boilerplate = if matches.is_present("no-preview-frame") {
                template::format_boilerplate_no_preview(infile)
            } else {
                template::format_boilerplate(infile)
            };
            
            let css: &str = template::CSS;
            let footer: &str = template::FOOTER;
            let prism: &str = prism::PRISM;
            let prismcss: &str = prism::CSS;

            file.write_all(boilerplate.as_bytes())
                .expect("could not write file");
            if highlight {
                file.write_all(prismcss.as_bytes())
                    .expect("could not highlight file");
            };
            file.write_all(css.as_bytes()).expect("could not read file");
            file.write_all(markdown.as_bytes())
                .expect("could not read file");
            if highlight {
                file.write_all(prism.as_bytes())
                    .expect("could not highlight file");
            };
            file.write_all(footer.as_bytes())
                .expect("could not write file");
        }
        Ok(())
    } 
    else {
        
        
        let updaters = Arc::new(Mutex::new(Vec::new()));
        let _watcher = spawn_watcher(Arc::clone(&updaters));

        let service = make_service_fn(|_| {
            let updaters = Arc::clone(&updaters);
            async {
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    router(Arc::clone(&updaters), req)
                }))
            }
        });

        let addr = std::net::SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10009);
        let server = Server::bind(&addr).serve(service);
        
        if browser { 
            let url = Url::parse("http://localhost:10009").unwrap();
               openurl::open(&url);
        }
        println!("listening on http://localhost:10009");
        server.await?;
        Ok(())
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

