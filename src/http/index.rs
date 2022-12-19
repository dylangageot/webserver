use super::{Body, Error, Headers, Message, Result, Status};
use ramhorns::{Content, Template};
use std::fs;
use std::path::{Path, PathBuf};

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

impl Index {
    fn render(&self) -> Result<String> {
        let tpl = Template::new(
            "\
<html>
        <head>
            <title>Index of {{path}}</title>
        </head>
        <body>
            <h2>Index of {{path}}</h2>
            <ul>{{#entries}}
                <li><a href=\"/{{url}}\">{{label}}</a></li>{{/entries}}
            </ul>
        </body>
</html>",
        )?;
        Ok(tpl.render(self))
    }
}

pub fn generate(base_path: PathBuf, path: PathBuf) -> Result<Message> {
    let path = base_path.join(
        path.strip_prefix(PathBuf::from("/"))
            .map_err(|_| Error::IndexGeneration("couldn't strip / from url".to_string()))?,
    );

    if !path.exists() {
        return Ok(Message::new(
            Status::NotFound,
            Some(Headers::from([(
                String::from("Content-Type"),
                String::from("text/html"),
            )])),
            Some(
                format!(
                    "\
<html>
        <head>
            <title>Not found</title>
        </head>
        <body>
            <h2>Not Found</h2>

            <p>Requested file or directory '{}' could not be found.</p>
        </body>
</html>",
                    path.to_string_lossy().to_string()
                )
                .parse()?,
            ),
        ));
    } else if path.is_file() {
        return Ok(Message::new(
            Status::Ok,
            Some(Headers::from([(
                String::from("Content-Type"),
                String::from("application/octet-stream"),
            )])),
            Some(Body::from(fs::read(path)?)),
        ));
    }

    let get_path: fn(path: &Path) -> Result<String> = |path: &Path| {
        Ok(path
            .to_str()
            .ok_or(Error::IndexGeneration(
                "couldn't get string from a path".to_string(),
            ))?
            .to_string())
    };
    let mut index = Index {
        path: get_path(
            path.strip_prefix(&base_path)
                .unwrap_or(PathBuf::from("/").as_path()),
        )?,
        entries: Vec::new(),
    };
    if let Some(parent_path) = path.parent() {
        index.entries.push(Entry {
            url: get_path(
                parent_path
                    .strip_prefix(&base_path)
                    .unwrap_or(PathBuf::from("").as_path()),
            )?,
            label: "..".to_string(),
        })
    }
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        let path = dir.path();
        let path = path
            .strip_prefix(&base_path)
            .map_err(|_| Error::IndexGeneration("couldn't strip base url from url".to_string()))?;
        index.entries.push(Entry {
            url: get_path(&path)?,
            label: dir
                .file_name()
                .to_str()
                .ok_or(Error::IndexGeneration(
                    "couldn't get string from a path".to_string(),
                ))?
                .to_string(),
        });
    }
    println!("{:#?}", index);

    Ok(Message::new(
        Status::Ok,
        Some(Headers::from([(
            String::from("Content-Type"),
            String::from("text/html"),
        )])),
        Some(index.render()?.parse()?),
    ))
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
