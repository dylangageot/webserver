use super::Body;

use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use ramhorns::{Content, Error, Template};

#[derive(Debug, Content)]
struct Entry {
    url: String,
    label: String,
}

#[derive(Debug, Content)]
struct Index {
    path: String,
    entries: Vec<Entry>,
}

const BASE_PATH: &str = "/home/gageotd";

const INDEX: &str = "\
<html>
        <head>
            <title>Index of {{path}}</title>
        </head>
        <body>
            <h2>Index of {{path}}</h2>{{#entries}}
            <a href=\"/{{url}}\">{{label}}</a><br/>{{/entries}}
        </body>
</html>";

impl Index {
    fn render(&self) -> Result<String, Error> {
        let tpl = Template::new(INDEX)?;
        Ok(tpl.render(self))
    }
}

pub fn display_dir(path: &str) -> std::io::Result<Body> {
    let path = PathBuf::from(BASE_PATH)
        .join(path.strip_prefix('/').unwrap())
        .canonicalize()
        .unwrap();
    let get_path = |path: &Path| path.to_str().unwrap().to_string();
    let mut index = Index {
        path: get_path(
            path.strip_prefix(BASE_PATH)
                .unwrap_or(PathBuf::from("/").as_path()),
        ),
        entries: Vec::new(),
    };
    if let Some(parent_path) = path.parent() {
        index.entries.push(Entry {
            url: get_path(
                parent_path
                    .strip_prefix(BASE_PATH)
                    .unwrap_or(PathBuf::from("").as_path()),
            ),
            label: "..".to_string(),
        })
    }
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        let path = dir.path();
        let path = path.strip_prefix(BASE_PATH).unwrap();
        index.entries.push(Entry {
            url: get_path(&path),
            label: dir.file_name().to_str().unwrap().to_string(),
        });
    }
    println!("{:#?}", index);
    Ok(Body::from_str(&index.render().unwrap()).unwrap())
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_index_generation() {
        assert_eq!(
            Index {
                path: String::from("/home"),
                entries: vec![
                    Entry {
                        url: String::from(""),
                        label: String::from("..")
                    },
                    Entry {
                        url: String::from("/home/user"),
                        label: String::from("user")
                    }
                ]
            }
            .render()
            .unwrap(),
            "\
<html>
        <head>
            <title>Index of /home</title>
        </head>
        <body>
            <h2>Index of /home</h2>
            <a href=\"/\">..</a><br/>
            <a href=\"//home/user\">user</a><br/>
        </body>
</html>"
        );
    }
}
