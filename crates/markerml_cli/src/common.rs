use anyhow::{anyhow, Context, Result};
use miette::{GraphicalReportHandler, NamedSource};
use std::fs::{self, File};
use std::path::Path;
use std::sync::LazyLock;

/// Error reporter used for pretty-printing miette errors
static ERROR_REPORTER: LazyLock<GraphicalReportHandler> =
    LazyLock::new(GraphicalReportHandler::new);

/// Checks whether the file exists
pub fn check_file_exists(filename: &Path) -> Result<()> {
    File::open(filename).with_context(|| format!("Couldn't open file {}", filename.display()))?;

    Ok(())
}

/// Reads given code file, parses it and return string with html
pub fn parse_file(filename: &Path) -> Result<String> {
    let content = fs::read_to_string(filename).context("Couldn't read file content")?;

    let html = match markerml::parse(&content) {
        Ok(html) => html,
        Err(err) => {
            let mut buffer = String::new();
            let err = miette::Error::from(err)
                .with_source_code(NamedSource::new(filename.display().to_string(), content));
            ERROR_REPORTER.render_report(&mut buffer, err.as_ref())?;
            println!("{}", buffer);

            return Err(anyhow!("Compilation error"));
        }
    };

    Ok(html)
}
