use color_eyre::eyre::Result;
use digital_garden::write::write;
use structopt::StructOpt;

/// A CLI for growing and curating a digital garden``
///
/// visit https://www.rustadventure.rs/garden for more!
 
#[derive(StructOpt, Debug)]
#[structopt(name = "garden")] // cli name
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// write something in your garden
    ///
    /// This command will open your $EDITOR, wait for
    /// you to write something, and then save the file
    /// to your garden
    Write {
        #[structopt(short, long)]
        title: Option<String>
    },
}

fn main() -> Result<()> {
    // throw if error
    color_eyre::install()?;
    // from_args is part of StructOpt struct
    let opt = Opt::from_args();
    dbg!(&opt);
    match opt.cmd {
        Command::Write { title } => write(title),
    }
}
