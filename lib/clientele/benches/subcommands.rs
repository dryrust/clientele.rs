// This is free and unencumbered software released into the public domain.

//! Synthetic Windows discovery measurements; timings are diagnostic, not gates.
//! Run with `cargo bench -p clientele --bench subcommands --no-default-features
//! --features std,subcommands --locked` on Windows.

fn main() {
    #[cfg(windows)]
    windows::run();
    #[cfg(not(windows))]
    println!("Windows discovery benchmark requires native Windows");
}

#[cfg(windows)]
mod windows {
    use clientele::{Subcommand, SubcommandsProvider};
    use std::{env, fs, hint::black_box, path::PathBuf, process::Command, time::Instant};
    use temp_dir::TempDir;

    const CHILD: &str = "CLIENTELE_DISCOVERY_BENCH_CHILD";
    const DIRECTORIES: usize = 6;
    const PER_DIRECTORY: usize = 16;
    const SAMPLES: usize = 9;

    pub fn run() {
        if env::var_os(CHILD).is_some() {
            let paths: Vec<_> = env::split_paths(&env::var_os("PATH").unwrap()).collect();
            let mut expected = Vec::new();
            for (directory, path) in paths.iter().take(DIRECTORIES).enumerate() {
                for index in 0..PER_DIRECTORY {
                    let name = format!("cmd{:03}.v1", directory * PER_DIRECTORY + index);
                    expected.push(Subcommand {
                        path: path.join(format!("bench-{name}.cmd")),
                        name,
                    });
                }
            }
            expected.push(Subcommand {
                name: "shared".into(),
                path: paths[0].join("bench-shared.cmd"),
            });
            // Warm filesystem caches, and check the fixture before timing it.
            assert_eq!(
                SubcommandsProvider::collect("bench-", 1).commands(),
                expected
            );
            for command in &expected {
                assert_eq!(
                    SubcommandsProvider::find("bench-", &command.name).as_ref(),
                    Some(command)
                );
            }
            let mut samples = Vec::new();
            for _ in 0..SAMPLES {
                let started = Instant::now();
                let result = black_box(SubcommandsProvider::collect(black_box("bench-"), 1));
                samples.push(started.elapsed());
                assert_eq!(result.commands(), expected);
            }
            samples.sort();
            println!(
                "discovery benchmark: directories={DIRECTORIES}+2 repeated, extensions=3, commands={}, samples={SAMPLES}, median_ms={:.3}, min_ms={:.3}, max_ms={:.3}",
                expected.len(),
                samples[SAMPLES / 2].as_secs_f64() * 1000.0,
                samples[0].as_secs_f64() * 1000.0,
                samples[SAMPLES - 1].as_secs_f64() * 1000.0,
            );
            return;
        }

        let root = TempDir::new().unwrap();
        let mut paths: Vec<PathBuf> = Vec::new();
        for directory in 0..DIRECTORIES {
            let path = std::path::absolute(root.child(format!("dir{directory}"))).unwrap();
            fs::create_dir(&path).unwrap();
            for index in 0..PER_DIRECTORY {
                let stem = format!("bench-cmd{:03}.v1", directory * PER_DIRECTORY + index);
                // Exact dotted files collide with stems but are not in PATHEXT.
                fs::write(path.join(&stem), "exact collision").unwrap();
                for extension in ["exe", "bat", "cmd"] {
                    fs::write(path.join(format!("{stem}.{extension}")), "fixture").unwrap();
                }
                fs::write(path.join(format!("unrelated{index}.txt")), "noise").unwrap();
            }
            for extension in ["bat", "cmd"] {
                fs::write(path.join(format!("bench-shared.{extension}")), "duplicate").unwrap();
            }
            paths.push(path);
        }
        paths.extend([paths[0].clone(), paths[3].clone()]);
        let output = Command::new(env::current_exe().unwrap())
            .env(CHILD, "1")
            .env("PATH", env::join_paths(paths).unwrap())
            .env("PATHEXT", ".CMD;.BAT;.EXE")
            .output()
            .unwrap();
        assert!(output.status.success(), "benchmark child: {output:?}");
        print!("{}", String::from_utf8(output.stdout).unwrap());
    }
}
