# redcord-rust
A rewrite of my redcord app (found [here](https://github.com/sethfl/redcord)) in pure Rust. It is currently in development, and thus nonfunctional. It is simply here to provide a backup for the project, as well as to allow for contributions. Any help would be appreciated. 

## Installation:

To install `redcord-rust`, you can just download a pre-compiled binary for your platform on the [releases page](https://github.com/sethfl/redcord-rust/releases).

## How to Use

The only setup required for this is to create a Discord Bot (instructions [here](https://discordpy.readthedocs.io/en/latest/discord.html) and setting the Bot's token as an environmental variable called `DISCORD_TOKEN`.

From there, you just run the binary and it will launch the bot.

### Troubleshooting

The only problems I have really come across with this is setting the environmental variable correctly. Try to confirm that your Bot Token is set as a variable called `DISCORD_TOKEN`. If there are any issues you come across, please contact me at sethaflynn@gmail.com

## For Developers:

### Dependencies
First, you need some system dependencies. The most important is Rust, which you can install using the `rustup` tool (read more [here](https://www.rust-lang.org/learn/get-started)).

Then, you need to install some extra dependencies for the libraries used (preferably through a package manager like apt, dnf, or pacman). These include:

- gcc
- rust
- cargo
- libssl-dev
- pkg-config
- libsodium-dev
- opus-tools
- libopus-dev
- ffmpeg

The exact name may be different for your operating system; find your equivalents.

### Getting the Source Code
You can clone the repository using the following command:

`git clone https://github.com/sethfl/redcord-rust.git`

or, you can just download a tarball from the [releases](https://github.com/sethfl/redcord-rust/releases) page. 

### Building the Source Code
You can build a binary by running this:
`cargo build`

You could also build an optimized binary (will take longer to compile) for yourself with the command

`cargo build --release`

### Installing the Source Code
You can install from source by running this command in the project folder:

`cargo install`

## Progress Plan:
The short-term goal of this project is to be able to send a random image from a user determined subreddit, into a discord chat. The long-term goal is to increase this functionality by increasing the support for the reddit API, allowing for features such as links to text posts, comments, viewing flairs, upvote counts, and more, all in discord! 

To see the current progress on development, take a look at the [project page](https://github.com/sethfl/redcord-rust/projects/1).
