use clap::Parser;
use std::fs;
use std::path::Path;
use std::error::Error;

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
        },
        RouterType::App => {
            return Err("App Router detected. This tool currently only supports Pages Router. App Router support is under development.".into());
        },
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
        Err("Neither Pages Router nor App Router detected. Please check your project structure.".into())
    }
}

fn get_routes(pages_dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let path = Path::new(pages_dir);
    if !path.exists() {
        return Err(format!("Directory '{}' not found", pages_dir).into());
    }
    Ok(get_files(path))
}

fn get_files(path: &Path) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(get_files(&path));
            } else if let Some(ext) = path.extension() {
                if ext == "tsx" || ext == "ts" {
                    if let Some(file_str) = path.to_str() {
                        files.push(file_str.to_string());
                    }
                }
            }
        }
    }
    files
}

fn generate_page_routes(files: Vec<String>) -> Vec<String> {
    let routes: Vec<String> = files
        .iter()
        .map(|f| file_to_url_path(f))
        .collect();
    
    let mut routes_with_file_name: Vec<String> = routes
        .iter()
        .map(|r| remove_filename(r))
        .collect();
    
    routes_with_file_name.sort();
    routes_with_file_name.dedup();
    
    routes_with_file_name
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
        format!("/{}", path_without_extension.replace('\\', "/"))
    }
}

fn remove_filename(path: &str) -> String {
    path.rsplit_once('/')
        .map(|(dir, _)| if dir.is_empty() { "/" } else { dir })
        .unwrap_or("/")
        .to_string()
}
