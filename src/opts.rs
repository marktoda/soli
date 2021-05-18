use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub enum Command {
    #[structopt(alias = "ls")]
    List,
    #[structopt(alias = "lsr")]
    ListRemote,

    #[structopt(alias = "i")]
    Install {
        // #[structopt(short = "v", long = "version")]
        version: String,
    },

    #[structopt(alias = "un")]
    Uninstall {
        // #[structopt(short = "v", long = "version")]
        version: String,
    },

    #[structopt(alias = "u")]
    Use {
        // #[structopt(short = "v", long = "version")]
        version: String,
    }
}

#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "Soli", about = "Options for the Soli CLI utility")]
pub struct Opt {
    #[structopt(parse(from_os_str), short = "d", long = "dir", default_value = r"~/.soli/")]
    pub dir: PathBuf,

    #[structopt(parse(from_os_str), short = "e", long = "exe_dir", default_value = r"~/.local/bin/")]
    pub executable_dir: PathBuf,

    #[structopt(subcommand)]
    pub cmd: Command,
}

impl Opt {
    pub fn get_dir(&self) -> PathBuf {
        PathBuf::from(shellexpand::tilde(self.dir.to_str().unwrap()).into_owned())
    }

    pub fn get_exe_dir(&self) -> PathBuf {
        PathBuf::from(shellexpand::tilde(self.executable_dir.to_str().unwrap()).into_owned())
    }
}
