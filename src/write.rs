use color_eyre::{eyre::WrapErr, Result};
use edit::{edit_file, Builder};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

const TEMPLATE: &[u8; 2] = b"# ";

pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<()> {
    let (mut file, filepath) = Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(&garden_path)
        .wrap_err("Failed to create temp file")?
        .keep()
        .wrap_err("Failed to keep temp file")?;

    file.write_all(TEMPLATE)?;
    edit_file(filepath)?;

    let mut contents = String::new();
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut contents)?;

    let document_title = title.or_else(|| {
        contents
            .lines()
            .find(|v| v.starts_with("# "))
            .map(|maybe_line| maybe_line.trim_start_matches("# ").to_string())
    });

    let filename = match document_title {
        Some(raw_title) => confirm_filename(&raw_title),
        None => ask_for_filename(),
    };

    todo!()
}

fn confirm_filename(raw_title: &str) -> Result<String> {
    loop {
        let result = rprompt::prompt_reply_stderr(&format!(
            "\
            current title: `{}`
            Do you want a different title? (y/N):",
            raw_title,
        ))
        .wrap_err("Failed to get input for y/n question")?;

        match result.as_str() {
            "y" | "Y" => break ask_for_filename(),
            "n" | "N" | "" => {
                break Ok(slug::slugify(raw_title));
            }
            _ => {}
        }
    }
}

fn ask_for_filename() -> Result<String> {
    rprompt::prompt_reply_stderr(
        "\
        EnterFilename
        >
        ",
    )
    .wrap_err("Failed to get filename")
}
