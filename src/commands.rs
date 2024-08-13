use anyhow::Result;
use clap::Subcommand;

mod apply;
mod create;
mod enter;
mod install;
mod remove;
mod run;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Create {
        name: String,

        #[clap(short, long, default_value = "debian")]
        distro: Option<String>,

        #[clap(short)]
        use_default: bool,
    },
    Enter {
        container: Option<String>,
    },
    Install {
        package: String,

        #[clap(short, long)]
        container: Option<String>,
    },
    Remove {
        package: String,

        #[clap(short, long)]
        container: Option<String>,
    },
    Apply {
        file: String,
    },
    Run {
        package: String,

        #[clap(short, long)]
        container: Option<String>,
    },
}

impl Commands {
    pub fn exec(&self) -> Result<()> {
        match self {
            Self::Create {
                name,
                distro,
                use_default,
            } => {
                create::create_container(
                    name.to_owned(),
                    distro.to_owned().unwrap_or("debian".to_string()),
                    use_default.to_owned(),
                )?;
            }
            Self::Install { package, container } => {
                install::install_command(package.to_owned(), container.to_owned())?;
            }
            Self::Run { package, container } => {
                run::run_command(container.to_owned(), package.to_owned())?;
            }
            Self::Enter { container } => {
                enter::enter_command(container.to_owned())?;
            }
            Self::Remove { package, container } => {
                remove::remove_command(package.to_owned(), container.to_owned())?;
            }
            Self::Apply { file } => {
                apply::apply_command(file.to_owned())?;
            }
        }
        Ok(())
    }
}
