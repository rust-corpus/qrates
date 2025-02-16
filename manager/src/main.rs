// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "corpus-manager",
    about = "Manager of the Rust corpus database."
)]
struct CorpusManagerArgs {
    #[structopt(
        parse(from_os_str),
        default_value = "CrateList.json",
        long = "crate-list-path",
        help = "The file specifying crates and their versions.",
        env = "QRATES_CRATE_LIST"
    )]
    crate_list_path: PathBuf,
    #[structopt(
        parse(from_os_str),
        default_value = "../workspace/database",
        long = "database",
        help = "The directory in which the database is stored.",
        env = "QRATES_DATABASE"
    )]
    database_root: PathBuf,
    #[structopt(
        parse(from_os_str),
        default_value = "../workspace",
        long = "workspace",
        help = "The directory in which all crates are compiled.",
        env = "QRATES_WORKSPACE"
    )]
    workspace: PathBuf,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "init", about = "Initialise the list of crates.")]
    Init {
        #[structopt(help = "How many top crates to download.")]
        top_count: usize,
        #[structopt(long, help = "Download all crate versions or only the newest one.")]
        all_versions: bool,
    },
    #[structopt(
        name = "init-all",
        about = "Initialise the list of crates with all crates."
    )]
    InitAll {
        #[structopt(long, help = "Download all crate versions or only the newest one.")]
        all_versions: bool,
    },
    #[structopt(name = "compile", about = "Compile the list of crates.")]
    Compile {
        #[structopt(long, help = "Should the extractor output also json, or only bincode?")]
        output_json: bool,
        #[structopt(
            default_value = "4000000000",   // 4 GB
            long = "memory-limit",
            help = "The memory limit that is set while building a crate. 0 means no limit."
        )]
        memory_limit: usize,
        #[structopt(
            long = "enable-networking",
            help = "Should the network be enabled while building a crate?"
        )]
        enable_networking: bool,
        #[structopt(
            long = "use-original-rustc",
            help = "Should use the original rustc instead of the extractor for building a crate?"
        )]
        use_original_rustc: bool,
        #[structopt(
            long = "purge-build-dir",
            help = "Should we purge the build directory before compiling the crate?"
        )]
        purge_build_dir: bool,
        #[structopt(
            default_value = "900",
            long = "compilation-timeout",
            help = "The compilation timeout in seconds. 0 means no timeout."
        )]
        compilation_timeout: u64,
        #[structopt(
            default_value = "5242880",   // 5 MB
            long = "max-log-size",
            help = "The maximum log size per build before it gets truncated (in bytes)."
        )]
        max_log_size: usize,
        #[structopt(
            long = "custom-cargo-registry",
            help = "Should we use a different cargo registry than crates.io?"
        )]
        custom_registry: Option<String>,
    },
    #[structopt(
        name = "check-compilation",
        about = "Show report about the compilation errors."
    )]
    CheckCompilation {
        #[structopt(
            long = "delete-failures",
            help = "Should we delete all crates that failed to compile?"
        )]
        delete_failures: bool,
    },
    #[structopt(
        name = "move-extracted",
        about = "Move deduplicated extracted facts to the specified folder."
    )]
    MoveExtracted {
        #[structopt(parse(from_os_str))]
        target_dir: PathBuf,
    },
    #[structopt(
        name = "update-database",
        about = "Scan the compiled crates and update the database."
    )]
    UpdateDatabase,
    #[structopt(name = "query", about = "Run a specific query.")]
    RunQuery {
        #[structopt(help = "The name of the query to run.")]
        query_name: String,
        #[structopt(
            parse(from_os_str),
            default_value = "../workspace/reports",
            long = "reports-path",
            help = "The directory in which the reports are saved.",
            env = "QRATES_REPORTS"
        )]
        report_path: PathBuf,
    },
}

fn main() {
    color_backtrace::install();
    {
        let timestamp = chrono::Utc::now().format("%Y%m%dT%H%M%S");
        let info_log_file = format!("log/info-{}.log", timestamp);
        let trace_log_file = format!("log/trace-{}.log", timestamp);
        use simplelog::*;
        fs::create_dir_all("log").unwrap();
        let mut loggers: Vec<Box<dyn SharedLogger>> = vec![
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                fs::File::create(&info_log_file).unwrap(),
            ),
            WriteLogger::new(
                LevelFilter::Trace,
                Config::default(),
                fs::File::create(&trace_log_file).unwrap(),
            ),
        ];
        let term_logger = TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        );
        loggers.push(term_logger as Box<dyn SharedLogger>);
        let logger = CombinedLogger::new(loggers);
        rustwide::logging::init_with(*logger);
    }
    let args = CorpusManagerArgs::from_args();
    match args.cmd {
        Command::Init {
            top_count,
            all_versions,
        } => {
            corpus_manager::initialise_with_top(&args.crate_list_path, top_count, all_versions);
        }
        Command::InitAll { all_versions } => {
            corpus_manager::initialise_with_all(&args.crate_list_path, all_versions);
        }
        Command::Compile {
            output_json,
            memory_limit,
            enable_networking,
            use_original_rustc,
            purge_build_dir,
            compilation_timeout,
            max_log_size,
            custom_registry,
        } => {
            let toolchain = {
                use std::io::Read;
                let mut file = std::fs::File::open("rust-toolchain")
                    .expect("Failed to open file “rust-toolchain”.");
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("Failed to read “rust-toolchain”.");
                let toolchain_toml = contents
                    .parse::<toml::Value>()
                    .expect("Failed to parse “rust-toolchain” as toml value");
                if let toml::Value::Table(table) = toolchain_toml {
                    if let Some(toml::Value::Table(toolchain_table)) = table.get("toolchain") {
                        if let Some(toml::Value::String(toolchain)) = toolchain_table.get("channel")
                        {
                            toolchain.to_owned()
                        } else {
                            panic!("Missing “channel” key in the “rust-toolchain” file.")
                        }
                    } else {
                        panic!("Missing “toolchain” table in the “rust-toolchain” file.")
                    }
                } else {
                    panic!("“rust-toolchain” file has to be a table")
                }
            };
            let memory_limit = if memory_limit == 0 {
                None
            } else {
                Some(memory_limit)
            };
            let timeout = if compilation_timeout == 0 {
                None
            } else {
                Some(Duration::from_secs(compilation_timeout))
            };
            corpus_manager::compile(
                &args.crate_list_path,
                &args.workspace,
                toolchain,
                max_log_size,
                memory_limit,
                timeout,
                enable_networking,
                output_json,
                use_original_rustc,
                purge_build_dir,
                custom_registry,
            );
        }
        Command::CheckCompilation { delete_failures } => {
            corpus_manager::check_compilation(&args.workspace, delete_failures);
        }
        Command::MoveExtracted { target_dir } => {
            corpus_manager::move_extracted(&args.workspace, &target_dir);
        }
        Command::UpdateDatabase => {
            corpus_manager::update_database(&args.workspace, &args.database_root);
        }
        Command::RunQuery {
            query_name,
            report_path,
        } => {
            corpus_manager::run_query(
                &query_name,
                &args.database_root,
                &report_path,
                &args.workspace,
                &args.crate_list_path,
            );
        }
    }
}
