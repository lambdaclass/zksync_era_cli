use crate::config::ZKSyncConfig;
use clap::Subcommand;

#[derive(Subcommand)]
pub(crate) enum Command {
    #[clap(about = "todo", visible_alias = "todo")]
    Todo,
}

impl Command {
    pub async fn run(self, _cfg: ZKSyncConfig) -> eyre::Result<()> {
        match self {
            Command::Todo => {}
        };
        Ok(())
    }
}
