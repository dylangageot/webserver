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

fn generate_not_found_page(relative_path: impl AsRef<Path>) -> Result<Message> {
    Ok(Message::new(
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
        <title>404 Not Found</title>
    </head>
    <body>
        <h2>Not Found</h2>

        <p>Requested file or directory '{}' could not be found.</p>
    </body>
</html>",
                relative_path.as_ref().to_string_lossy().to_string()
            )
            .parse()?,
        ),
    ))
}

fn generate_file_response(absolute_path: impl AsRef<Path>) -> Result<Message> {
    Ok(Message::new(
        Status::Ok,
        Some(Headers::from([(
            String::from("Content-Type"),
            String::from("application/octet-stream"),
        )])),
        Some(Body::from(fs::read(absolute_path)?)),
    ))
}

fn generate_index_page(
    base_path: impl AsRef<Path>,
    relative_path: impl AsRef<Path>,
    absolute_path: impl AsRef<Path>,
) -> Result<Message> {
    let mut index = Index {
        path: relative_path.as_ref().to_string_lossy().to_string(),
        entries: Vec::new(),
    };
    if let Some(parent_path) = absolute_path.as_ref().parent() {
        println!("{:#?}", parent_path);
        if let Ok(parent_path) = parent_path.strip_prefix(&base_path) {
            index.entries.push(Entry {
                url: parent_path.to_string_lossy().to_string(),
                label: String::from(".."),
            })
        }
    }
    for entry in fs::read_dir(absolute_path)? {
        let dir = entry?;
        let path = dir.path();
        let path = path
            .strip_prefix(&base_path)
            .map_err(|_| Error::IndexGeneration("couldn't strip base url from url".to_string()))?;
        index.entries.push(Entry {
            url: path.to_string_lossy().to_string(),
            label: dir.file_name().to_string_lossy().to_string(),
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

pub fn generate(base_path: impl AsRef<Path>, relative_path: impl AsRef<Path>) -> Result<Message> {
    let absolute_path = base_path.as_ref().join(
        relative_path
            .as_ref()
            .strip_prefix(PathBuf::from("/"))
            .map_err(|_| Error::IndexGeneration(String::from("couldn't strip / from url")))?,
    );
    if !absolute_path.exists() {
        return generate_not_found_page(&relative_path);
    } else if absolute_path.is_file() {
        return generate_file_response(&absolute_path);
    }
    generate_index_page(base_path, relative_path, absolute_path)
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
