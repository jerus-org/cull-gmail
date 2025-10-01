use clap::{Parser, Subcommand};

mod list_cli;

use cull_gmail::Credential;
use google_gmail1::{Gmail, yup_oauth2::ApplicationSecret};
use list_cli::ListCli;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List messages
    #[clap(name = "list")]
    List(ListCli),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let mut logging = get_logging(args.logging.log_level_filter());
    logging.init();

    run(args).await;
    // match run(args) {
    //     Ok(_) => {}
    //     Err(e) => {
    //         if let Some(src) = e.source() {
    //             log::error!("{e}: {src}");
    //             eprintln!("{e}: {src}");
    //         } else {
    //             log::error!("{e}");
    //             eprintln!("{e}");
    //         }
    //         std::process::exit(101);
    //     }
    // }
}

async fn run(args: Cli) {
    let secret: ApplicationSecret = Credential::load_json_file("credential.json").into();

    let auth = cull_gmail::get_auth(secret).await;
    let client = cull_gmail::get_client();

    let hub = Gmail::new(client, auth);

    if let Some(cmds) = args.command {
        match cmds {
            Commands::List(list_cli) => list_cli.run(hub).await,
        }
    }
    // Ok(())
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
