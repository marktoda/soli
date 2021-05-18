use anyhow::Result;
use structopt::StructOpt;

mod opts;
use opts::{Command, Opt};

mod versions;
use versions::{
    get_current_version, get_local_versions, get_remote_versions, install_version,
    uninstall_version, use_version,
};

fn list(opt: &Opt) -> Result<()> {
    let current_version = get_current_version(&opt.get_exe_dir())?;
    for version in get_local_versions(&opt.get_dir())? {
        if version == current_version {
            println!(">> {}", version.as_str());
        } else {
            println!("   {}", version.as_str());
        }
    }

    Ok(())
}

async fn list_remote() -> Result<()> {
    for version in get_remote_versions().await? {
        println!("{}", version.as_str());
    }
    Ok(())
}

async fn install(opt: &Opt, version: &str) -> Result<()> {
    println!("Installing {}...", version);

    let install_res = install_version(&opt.get_dir(), version).await;
    if install_res.is_err() {
        eprintln!("{:?}", install_res.unwrap_err());
    }

    println!("Version {} installed.", version);
    Ok(())
}

fn uninstall(opt: &Opt, version: &str) -> Result<()> {
    uninstall_version(&opt.get_dir(), version)?;
    println!("Version {} uninstalled.", version);
    Ok(())
}

fn use_new_version(opt: &Opt, version: &str) -> Result<()> {
    use_version(&opt.get_dir(), &opt.get_exe_dir(), version)?;
    println!("Now using {}", version);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::from_args();

    match opt.cmd {
        Command::List => {
            list(&opt)?;
        }
        Command::ListRemote => {
            list_remote().await?;
        }
        Command::Install { ref version } => {
            install(&opt, version.clone().as_str()).await?;
        }
        Command::Uninstall { ref version } => {
            uninstall(&opt, version.clone().as_str())?;
        }
        Command::Use { ref version } => {
            use_new_version(&opt, version.clone().as_str())?;
        },
        Command::GetConfig => {
            println!("Dir: {:?}", &opt.get_dir());
            println!("Exe Dir: {:?}", &opt.get_exe_dir());
        }
    }

    Ok(())
}
