use clientele::{
    crates::clap::{self, CommandFactory, Parser, Subcommand},
    manpages::Man,
    StandardOptions,
};
use std::io::{self, Write};

#[derive(Parser)]
#[command(
    name = "demo-cli",
    version = "1.2.3",
    about = "Manage example resources"
)]
#[command(disable_version_flag = true)]
struct Options {
    #[command(flatten)]
    flags: StandardOptions,
    /// Output format
    #[arg(long, value_parser = ["json", "text"], default_value = "text")]
    format: String,
    #[arg(long, hide = true)]
    internal_switch: bool,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Inspect the configuration
    Config {
        /// Configuration path
        #[arg(long)]
        path: Option<std::path::PathBuf>,
    },
}

#[test]
fn renders_command_metadata_options_and_subcommand_references() {
    let page = Man::new(Options::command());
    let mut output = Vec::new();
    page.render(&mut output).unwrap();
    let roff = String::from_utf8(output).unwrap();
    for expected in [
        "NAME",
        "SYNOPSIS",
        "OPTIONS",
        "SUBCOMMANDS",
        "Manage example resources",
        "1.2.3",
        r"\-\-verbose",
        r"\-\-format",
        "json",
        "text",
        "config",
        "Inspect the configuration",
    ] {
        assert!(roff.contains(expected), "missing {expected} in {roff}");
    }
    assert!(!roff.contains("internal"));
    assert_eq!(page.get_filename(), "demo-cli.1");
}

#[test]
fn writes_a_custom_section_and_propagates_output_errors() {
    let directory = temp_dir::TempDir::new().unwrap();
    let page = Man::new(Options::command()).section("8");
    let path = page.generate_to(directory.path()).unwrap();
    assert_eq!(path.file_name().unwrap(), "demo-cli.8");
    assert!(std::fs::read_to_string(path).unwrap().contains("SYNOPSIS"));
    assert!(page.generate_to(directory.child("missing")).is_err());

    struct BrokenWriter;
    impl Write for BrokenWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::ErrorKind::BrokenPipe.into())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    assert_eq!(
        page.render(&mut BrokenWriter).unwrap_err().kind(),
        io::ErrorKind::BrokenPipe
    );
}
