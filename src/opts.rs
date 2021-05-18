use std::path::PathBuf;
use structopt::StructOpt;
use directories::ProjectDirs;

#[derive(Clone, Debug, StructOpt)]
pub enum Command {
    #[structopt()]
    GetConfig,
    #[structopt(alias = "ls")]
    List,
    #[structopt(alias = "lsr")]
    ListRemote,

    #[structopt(alias = "i")]
    Install {
        version: String,
    },

    #[structopt(alias = "un")]
    Uninstall {
        version: String,
    },

    #[structopt(alias = "u")]
    Use {
        version: String,
    },
}

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "Soli", about = "Options for the Soli CLI utility")]
pub struct Opt {
    #[structopt(
        parse(from_os_str),
        short = "d",
        long = "dir",
    )]
    pub dir: Option<PathBuf>,

    #[structopt(
        parse(from_os_str),
        short = "e",
        long = "exe_dir",
        default_value = r"~/.local/bin/"
    )]
    pub executable_dir: PathBuf,

    #[structopt(subcommand)]
    pub cmd: Command,
}

impl Opt {
    pub fn get_dir(&self) -> PathBuf {
        self.dir.as_ref().map_or(
            ProjectDirs::from("com", "Soli", "soli").expect("Invalid project dirs").data_dir().to_path_buf(),
            |dir| PathBuf::from(shellexpand::tilde(dir.to_str().unwrap()).into_owned()))
    }

    pub fn get_exe_dir(&self) -> PathBuf {
        PathBuf::from(shellexpand::tilde(self.executable_dir.to_str().unwrap()).into_owned())
    }
}
