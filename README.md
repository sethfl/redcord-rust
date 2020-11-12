# redcord

[![Build Status](https://travis-ci.org/sethfl/redcord.svg?branch=main)](https://travis-ci.org/sethfl/redcord)

`redcord` is a Discord bot that can send media from Reddit right into your Discord server. It does this by recieving a command from users, making a request to the public Reddit API, parsing recieved `.json` files, and then replying to the user with the data.

## Commands:

#### `-random [subreddit]`
This command, when sent into a sever with `redcord` present, triggers the bot to send a media URL from a random post on the subreddit entered with the command. The only other outputs are either a link to a comment (which is the case when the random post recieved does not contain a media URL), or `null` when the subreddit requested does not exist.

## Installation:

To install `redcord`, you can just download a pre-compiled binary for your platform on the [releases page](https://github.com/sethfl/redcord/releases).

## How to Use

The only setup required for this is to create a Discord Bot (instructions [here](https://discordpy.readthedocs.io/en/latest/discord.html)) and then setting the Bot's token as an environmental variable called `DISCORD_TOKEN` (this is done on linux with the command `export DISCORD_TOKEN=[BOT_TOKEN_HERE]`).

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

`git clone https://github.com/sethfl/redcord.git`

or, you can just download a tarball from the [releases](https://github.com/sethfl/redcord/releases) page. 

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
