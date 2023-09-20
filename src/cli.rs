use clap::Command;

pub fn cli() -> Command {
    Command::new("timy")
        .bin_name("timy")
        .subcommand_required(true)
        .version("")
        .subcommand(
            Command::new("print")
                .about("print a markdown bullet list from the output of 'zeitliste --csv'\ne.g. 'zeitliste --csv | timy -p'")
                .short_flag('p')
                .long_flag("pipe-into-markdown")
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("sctime-home")
                .about("sctime-home")
                .short_flag('s')
                .long_flag("sctime-home")
                .arg_required_else_help(false),
        )
}
