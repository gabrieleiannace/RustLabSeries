use clap::Parser;

#[derive(Parser, Debug)] // requires `derive` feature
pub enum CommandsOption {
    #[command(name = "new")]
    /// Create a new board
    CreateBoard(CreateBoardArgs),
    #[command(name = "add")]
    /// Add a boat
    AddBoat(AddBoatArgs),
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct CreateBoardArgs {
    #[arg()]
    /// The file name
    file_name: String,

    #[arg()]
    /// Number of boat with a ,
    boat_string: String,
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct AddBoatArgs {
    #[arg()]
    /// The file name
    file_name: String,

    #[arg()]
    /// Box number and direction [ H / V ]
    box_and_direction: String,
}