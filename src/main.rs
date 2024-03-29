use clap::{Parser, Subcommand};
use configuration::{Configuration, Player, Port};
use plex_client::PlexClient;

mod configuration;
mod plex_client;

/// Controls Plexamp Media Playback
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Playback subcommands
    Playback {
        /// The target player
        #[arg(short, long, default_value = "default")]
        player: String,
        // Command to be ran on the target player
        #[command(subcommand)]
        command: PlaybackCommands,
    },
    /// Configuration subcommands
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
}

#[derive(Subcommand, Clone)]
enum ConfigCommand {
    /// Adds a player target to the configuration
    Add {
        // Name of the player
        #[arg(short, long, default_value = "default")]
        name: String,
        // Hostname of the player
        host: String,
        // Port of the player
        port: Port,
    },
    /// Deletes a configured player target
    Delete {
        // Name of the player to delete
        name: String,
    },
    /// List configured player target
    List,
    /// Sets default player target
    Default {
        /// Name of the player to be set as default
        name: String,
    },
}

#[derive(Subcommand, Clone)]
enum PlaybackCommands {
    Play,
    Pause,
    Stop,
    Next,
    Previous,
    Forward,
    Backward,
}

fn main() {
    env_logger::init();
    let args = Cli::parse();
    let mut config: Configuration = Configuration::load();

    match args.command {
        Some(Commands::Playback { player, command }) => {
            let player_struct = config.get_player(&player).expect("Invalid player");

            run_playback_command(command, player_struct)
        }
        Some(Commands::Config { command }) => match command {
            ConfigCommand::Add { name, host, port } => {
                config.add_player(name, host, port).store();
            }
            ConfigCommand::Delete { name } => {
                config.remove_player(name).store();
            }
            ConfigCommand::List => {
                println!("Configured players:");
                for (name, Player { host, port, .. }) in config.players.iter() {
                    if config.is_default(name) {
                        println!("\t{name} -> {host}:{port} [DEFAULT]");
                    } else {
                        println!("\t{name} -> {host}:{port}");
                    }
                }
            }
            ConfigCommand::Default { name } => {
                if config.set_default(name.clone()) {
                    config.store();
                } else {
                    println!("No such player named {name}")
                }
            }
        },
        None => {
            println!("No command provided");
        }
    }
}

fn run_playback_command(command: PlaybackCommands, player: &Player) {
    let client = PlexClient {
        player: player.clone(),
    };
    match command {
        PlaybackCommands::Play => client.play(),
        PlaybackCommands::Pause => client.pause(),
        PlaybackCommands::Stop => client.stop(),
        PlaybackCommands::Next => client.next(),
        PlaybackCommands::Previous => client.previous(),
        PlaybackCommands::Forward => client.forward(),
        PlaybackCommands::Backward => client.backward(),
    }
}
