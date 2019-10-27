#[allow(dead_code)]
mod devoxx;
mod ui;

use std::io;
use std::io::Write;

use structopt::StructOpt;

fn main() -> Result<(), failure::Error> {
    // Parse any command-line arguments
    let options = Options::from_args();

    // Run the app
    let result = ui::screen::run(options);

    // Write any errors on exit to the screen
    if let Err(e) = result {
        let mut stdout = io::stdout();
        stdout.write_all(format!("{:#?}", e).as_bytes())?;
    }
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "devoxx-schedule", about = "A command line tool to browse the Devoxx schedule")]
pub struct Options {
    #[structopt(short, long)]
    /// Uses the schedule from local disk, instead of the Devoxx API
    pub offline: bool
}
