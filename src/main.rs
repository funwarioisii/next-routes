use clap::Parser;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    src: bool,
}

#[derive(Debug)]
enum RouterType {
    Pages,
    App,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let router_type = detect_router_type(args.src)?;
    match router_type {
        RouterType::Pages => {
            let pages_dir = if args.src { "src/pages" } else { "pages" };
            let file_names = get_routes(pages_dir)?;
            let routes = generate_page_routes(file_names);
            for route in routes {
                println!("{}", route);
            }
        }
        RouterType::App => {
            return Err("App Router detected. This tool currently only supports Pages Router. App Router support is under development.".into());
        }
    }

    Ok(())
}

fn detect_router_type(src: bool) -> Result<RouterType, Box<dyn Error>> {
    let app_dir = if src { "src/app" } else { "app" };
    let pages_dir = if src { "src/pages" } else { "pages" };

    if Path::new(app_dir).exists() {
        Ok(RouterType::App)
    } else if Path::new(pages_dir).exists() {
        Ok(RouterType::Pages)
    } else {
        Err(
            "Neither Pages Router nor App Router detected. Please check your project structure."
                .into(),
        )
    }
}

fn get_routes(pages_dir: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let path = Path::new(pages_dir);
    if !path.exists() {
        return Err(format!("Directory '{}' not found", pages_dir).into());
    }
    Ok(get_files(path))
}

fn get_files(path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(get_files(&path));
            } else if let Some(ext) = path.extension() {
                if ext == "tsx" || ext == "ts" {
                    files.push(path);
                }
            }
        }
    }
    files
}

const PAGES_RESERVED_ROUTES: [&str; 5] = ["/404", "/500", "/_app", "/_document", "/_error"];

fn generate_page_routes(files: Vec<PathBuf>) -> Vec<String> {
    let mut routes: Vec<String> = files
        .iter()
        .map(|f| file_to_url_path(f.to_str().unwrap()))
        .filter(|f| !PAGES_RESERVED_ROUTES.contains(&f.as_str()))
        .collect();

    routes.sort();
    routes.dedup();

    routes
}

fn file_to_url_path(file: &str) -> String {
    let path = file
        .strip_prefix("src/pages/")
        .or_else(|| file.strip_prefix("pages/"))
        .unwrap_or(file);

    let path_without_extension = path
        .strip_suffix(".tsx")
        .or_else(|| path.strip_suffix(".ts"))
        .unwrap_or(path);

    if path_without_extension == "index" {
        "/".to_string()
    } else {
        let url = format!("/{}", path_without_extension.replace('\\', "/"));
        if url.ends_with("/index") {
            url.strip_suffix("/index").unwrap().to_string()
        } else {
            url
        }
    }
}
