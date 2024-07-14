use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{ffi::OsStr, path::Path};

use heck::ToSnakeCase;
use heck::ToUpperCamelCase;
use regex::Regex;
use scraper::node::Element;
use walkdir::WalkDir;

const ICON_TEMPLATE: &str = r#"#[derive(Copy, Clone, Debug, PartialEq)]
pub struct {ICON_NAME};
impl IconShape for {ICON_NAME} {
    fn content(&self) -> &'static str {
        {ICON_CONTENT}
    }
}
"#;

pub fn create_icon_file(svg_path: &str, output_path: &str, icon_prefix: &str) {
    let files = collect_svg_files(svg_path, icon_prefix);

    let icon_file = files
        .into_iter()
        .map(|file| {
            let svg_str = fs::read_to_string(&file).unwrap();
            let icon_name = icon_name(&file, icon_prefix);

            ICON_TEMPLATE
                .replace("{ICON_NAME}", &format!("{}{}", icon_prefix, &icon_name))
                .replace("{ICON_CONTENT}", &format!("r#\"{svg_str}\"#"))
        })
        .collect::<Vec<_>>()
        .join("\n");

    // write to file
    let mut file = File::create(output_path).unwrap();
    file.write_all(
        format!(
            "{}\n\n{}",
            "use super::super::IconShape;\nuse freya::prelude::*;", icon_file
        )
        .as_bytes(),
    )
    .unwrap();
    file.flush().unwrap();
}

fn collect_svg_files(svg_path: &str, icon_prefix: &str) -> Vec<PathBuf> {
    let dir_entries = WalkDir::new(svg_path)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .collect::<Vec<_>>();

    dir_entries
        .into_iter()
        .filter(|e| match icon_prefix {
            "Go" => {
                let re = Regex::new(r".*-16.svg$").unwrap();
                return re.is_match(e.path().to_str().unwrap());
            }
            "Md" => {
                let split_vec = e.path().components().collect::<Vec<_>>();
                return split_vec.iter().any(|c| c.as_os_str() == "materialicons")
                    && e.file_name().to_str().unwrap() == "24px.svg";
            }
            _ => return e.path().extension() == Some(OsStr::new("svg")),
        })
        .map(|dir| PathBuf::from(dir.path()))
        .collect::<Vec<_>>()
}

fn icon_name(path: &Path, icon_prefix: &str) -> String {
    match icon_prefix {
        "Go" => {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let name = filename.split('.').next().unwrap();
            name.replace("-16", "").to_upper_camel_case()
        }
        "Md" => {
            let split_vec = path.components().collect::<Vec<_>>();
            let name = split_vec[split_vec.len() - 3];
            name.as_os_str().to_str().unwrap().to_upper_camel_case()
        }
        _ => {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let name = filename.split('.').next().unwrap();
            name.to_upper_camel_case()
        }
    }
}
