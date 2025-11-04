use clap::{Parser, Subcommand};
use ngram::client::Client;
use ngram::server::Server;

// TODO:
// Fill out the `Args` struct to parse the command line arguments. You may find clap "subcommands"
// helpful.
/// An archive service allowing publishing and searching of books
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Server {
        port: u16,
    },
    Client {
        address: String,
        port: u16,
        #[command(subcommand)]
        operation: ClientOperation,
    },
}

#[derive(Subcommand, Debug)]
enum ClientOperation {
    Publish { path: String },
    Search { word: String },
    Retrieve { id: usize },
}

// TODO:
// Inspect the contents of the `args` struct that has been created from the command line arguments
// the user passed. Depending on the arguments, either start a server or make a client and send the
// appropriate request. You may find it helpful to print the request response.
fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Server { port } => {
            println!("Starting server on port {}...", port);
            let server = Server::new();
            server.run(port);
        }
        Commands::Client {
            address,
            port,
            operation,
        } => {
            let client = Client::new(&address, port);

            let response = match operation {
                ClientOperation::Publish { path } => {
                    println!("Publishing document from {}...", path);
                    client.publish_from_path(&path)
                }
                ClientOperation::Search { word } => {
                    println!("Searching for '{}'...", word);
                    client.search(&word)
                }
                ClientOperation::Retrieve { id } => {
                    println!("Retrieving document {}...", id);
                    client.retrieve(id)
                }
            };

            match response {
                Some(resp) => println!("{:?}", resp),
                None => println!("Failed to get response from server"),
            }
        }
    }
}
