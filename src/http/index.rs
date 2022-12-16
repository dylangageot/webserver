use super::Body;

use std::path::Path;
use std::{fmt::Debug, fs, io::Write};

pub fn display_dir(path: &str) -> std::io::Result<Body> {
    let path = format!(".{}", path);
    let path = Path::new(&path);
    let mut body = Vec::new();

    body.write_fmt(format_args!(
        "\
<html>
        <head>
            <title>Index of {0}</title>
        </head>
        <body>
            <h2>Index of {0}</h2>
            <a href=\"{1}\">..</a><br/>
            ",
        path.to_str().unwrap(),
        path.parent().unwrap().to_str().unwrap()
    ))
    .unwrap();

    for entry in fs::read_dir(path)? {
        let dir = entry?;
        body.write_fmt(format_args!(
            "\
            <a href=\".{0}/{1}\">{1}</a><br/>
        ",
            if path.to_str().unwrap() == "/" {
                ""
            } else {
                path.to_str().unwrap()
            },
            dir.file_name().to_string_lossy().to_string(),
        ))
        .unwrap();
    }

    body.write_fmt(format_args!(
        "\
        </body>
</html>
"
    ))
    .unwrap();

    Ok(Body::from(body))
}
