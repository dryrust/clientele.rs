use clientele::{
    StandardOptions,
    completions::{Shell, generate, generate_to},
    crates::clap::{self, CommandFactory, Parser, Subcommand, ValueEnum},
};

#[derive(Parser)]
#[command(name = "internal-name")]
struct Options {
    #[command(flatten)]
    flags: StandardOptions,
    #[arg(long, value_enum)]
    format: Option<Format>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Clone, ValueEnum)]
enum Format {
    Json,
    Text,
}

#[derive(Subcommand)]
enum Command {
    /// Inspect the configuration
    Config {
        #[arg(long)]
        path: Option<std::path::PathBuf>,
    },
}

#[test]
fn generates_each_supported_shell_from_the_consumers_command() {
    for shell in Shell::value_variants() {
        let mut output = Vec::new();
        generate(*shell, &mut Options::command(), "demo-cli", &mut output);
        let script = String::from_utf8(output).unwrap();
        for expected in ["demo-cli", "verbose", "format", "config", "path"] {
            assert!(script.contains(expected), "{shell}: missing {expected}");
        }
        if *shell == Shell::Bash {
            for value in ["json", "text"] {
                assert!(script.contains(value), "missing possible value {value}");
            }
        }
        if *shell == Shell::Zsh {
            assert!(script.contains("#compdef demo-cli"));
        }
    }
}

#[test]
fn writes_a_completion_file_and_reports_missing_directories() {
    let directory = temp_dir::TempDir::new().unwrap();
    let path = generate_to(
        Shell::Bash,
        &mut Options::command(),
        "demo-cli",
        directory.path(),
    )
    .unwrap();
    let script = std::fs::read_to_string(path).unwrap();
    assert!(script.contains("--verbose"));
    assert!(script.contains("--format"));
    assert!(script.contains("config"));
    assert!(
        generate_to(
            Shell::Bash,
            &mut Options::command(),
            "demo-cli",
            directory.child("missing"),
        )
        .is_err()
    );
}
