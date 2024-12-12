mod commands;
mod default_scenarios;
mod util;

use std::sync::LazyLock;

use commands::{ContenderCli, ContenderSubcommand, SpamCommandArgs};
use contender_core::{db::DbOps, generator::RandSeed};
use contender_sqlite::SqliteDb;

static DB: LazyLock<SqliteDb> = std::sync::LazyLock::new(|| {
    SqliteDb::from_file("contender.db").expect("failed to open contender.db")
});

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ContenderCli::parse_args();
    let _ = DB.create_tables(); // ignore error; tables already exist
    let db = DB.clone();

    match args.command {
        ContenderSubcommand::Setup {
            testfile,
            rpc_url,
            private_keys,
            min_balance,
            seed,
            num_signers_per_pool,
        } => {
            if num_signers_per_pool.is_none() {
                eprintln!("Warning: --signers-per-pool (-n) not specified; defaulting to 1");
            }
            commands::setup(
                &db,
                testfile,
                rpc_url,
                private_keys,
                min_balance,
                RandSeed::seed_from_str(&seed),
                num_signers_per_pool.unwrap_or(1),
            )
            .await?
        }

        ContenderSubcommand::Spam {
            testfile,
            rpc_url,
            builder_url,
            txs_per_block,
            txs_per_second,
            duration,
            seed,
            private_keys,
            disable_reports,
            min_balance,
        } => {
            commands::spam(
                &db,
                SpamCommandArgs {
                    testfile,
                    rpc_url,
                    builder_url,
                    txs_per_block,
                    txs_per_second,
                    duration,
                    seed,
                    private_keys,
                    disable_reports,
                    min_balance,
                },
            )
            .await?
        }

        ContenderSubcommand::Report { id, out_file } => commands::report(&db, id, out_file)?,

        ContenderSubcommand::Run {
            scenario,
            rpc_url,
            private_key,
            interval,
            duration,
            txs_per_duration,
        } => {
            commands::run(
                &db,
                scenario,
                rpc_url,
                private_key,
                interval,
                duration,
                txs_per_duration,
            )
            .await?
        }
    }
    Ok(())
}
