use super::Body;

use std::fs;
use std::path::Path;
use std::str::FromStr;

use ramhorns::{Content, Error, Template};

#[derive(Content)]
struct Entry {
    url: String,
    label: String,
}

#[derive(Content)]
struct Index {
    path: String,
    entries: Vec<Entry>,
}

const INDEX: &str = "\
<html>
        <head>
            <title>Index of {{path}}</title>
        </head>
        <body>
            <h2>Index of {{path}}</h2>
            {{#entries}}<a href=\"{{url}}\">{{label}}</a><br/>{{/entries}}
        </body>
</html>";

impl Index {
    fn render(&self) -> Result<String, Error> {
        let tpl = Template::new(INDEX)?;
        Ok(tpl.render(self))
    }
}

pub fn display_dir(path: &str) -> std::io::Result<Body> {
    let path = format!(".{}", path);
    let mut index = Index {
        path: path.to_string(),
        entries: Vec::new(),
    };
    for entry in fs::read_dir(Path::new(&path))? {
        let dir = entry?;
        index.entries.push(Entry {
            url: dir.path().to_str().unwrap().to_string(),
            label: dir.file_name().to_str().unwrap().to_string(),
        });
    }
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
                        url: String::from("./"),
                        label: String::from("..")
                    },
                    Entry {
                        url: String::from("./home/user"),
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
            <a href=\"./\">..</a><br/>
            <a href=\"./home/user\">user</a><br/>
        </body>
</html>"
        );
    }
}
