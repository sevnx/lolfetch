# Lolfetch

Lolfetch is a command-line League of Legends information tool written in Rust.  
Communication with the Riot API is done through the [Riven crate](https://github.com/MingweiSamuel/Riven).

Inspired by [Neofetch](https://github.com/dylanaraps/neofetch) and [onefetch](https://github.com/o2sh/onefetch)

## Installation

### Pre-requisites

To use this program, one must have a Riot API key which can be obtained from the [Riot Developer Portal](https://developer.riotgames.com/).

You will then have to either set the `RIOT_API_KEY` environment variable or pass the key as an argument to the program.

> **Note**: The basic API key is limited to 100 requests every 2 minutes. Thanks to the `Riven` crate, the program will automatically handle the rate limiting,
> but it might take some time to fetch an entire season's worth of ranked information.

### Cargo installation

The program can be installed using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```sh
cargo install lolfetch
```

### Building from source

The program can also be built from source. This requires the Rust toolchain to be installed (which can be acquired through [rustup](https://rustup.rs/)).

```sh
git clone https://github.com/sevnx/lolfetch.git
cd lolfetch
cargo install --path .
```

## Example

The following command will display the ranked information of the player with the given `RIOT_ID` in the given `REGION`.

```sh
RIOT_API_KEY=<API_KEY> lolfetch display ranked <RIOT_ID> <REGION>
```

This will output something like this (with pretty colors):

```plaintext
➜ RIOT_API_KEY=<API_KEY> lolfetch display ranked "hide on bush#KR1" KR
                                                  
                                                  
                                                  
   %*+*%                                   %*+%        Summoner: hide on bush#KR1
    *--+                                  %;:;*        Rank: GRANDMASTER - 1066 LP
     +--+                                 ;--*         
     +;--+                               +-;;*         Match History
 S%*%%-;-;*+                          *+*--;;% %%S     -----
  +;-*+--;+;;%                      S*;++;--**-;*      23:34 - W - MID - Ziggs    - 3/2/8    - 5.5 KDA - 8.1 CS/M - GD@15: +803
  +;:-*+--+%;-+                    %;-+%+--**--;       27:50 - L - MID - Yone     - 2/7/4    - 0.9 KDA - 8.4 CS/M - GD@15: -943
   +;-;;;:-%$%;+*+;;    -*    -;++*++%$*:-;;--;        30:45 - W - MID - Tristana - 7/4/5    - 3.0 KDA - 9.1 CS/M - GD@15: -583
    ++-:---++*%;++++;*+-;*;-*+;+++++%*+;--::-++        15:14 - W - MID - Smolder  - 3/0/1    - PERFECT - 10.0 CS/M - GD@15: +1988
     +**;--;;;*+-+*%S;*+;;;*++S%*+-**;;;-;;**          26:52 - L - MID - Smolder  - 4/6/4    - 1.3 KDA - 9.4 CS/M - GD@15: -179
        +;;-;*;;-;%S$S+**++--SSS*;-;;*;;;;*            
         ;:+;-:::+*%SSS*+*-+SSS%*;:::-++:;             Champion Stats (last 10 games)
         +-;%*+--:-;*+%#%;%$***;-:--+%%-;              -----
           ;-;+*;--::;++$SS++;::--+*+--+               Aurelion Sol -  33% WR - 2.2 KDA - 8.0 CS/M - 3 Played
            +-::---;;-;;*$+;--;;---::-                 Smolder      -  50% WR - 2.0 KDA - 9.6 CS/M - 2 Played
               +++;-;;;++%+;;;;-;+++                   Yone         -  50% WR - 1.9 KDA - 8.0 CS/M - 2 Played
                     --;;-;;-;                         Ziggs        - 100% WR - 5.5 KDA - 8.1 CS/M - 1 Played
                       -  ;-                           Nasus        -   0% WR - 0.6 KDA - 7.2 CS/M - 1 Played
                                                  
                                         
```

## Usage

The output of the program can be customized using different CLI options.

Output of `lolfetch help`:

```plaintext
A command-line League of Legends information tool

Usage: lolfetch [OPTIONS] <COMMAND>

Commands:
  cache    Cache management
  display  Default lolfetch mode
  help     Print this message or the help of the given subcommand(s)

Options:
      --verbose  Verbose mode
  -h, --help     Print help
  -V, --version  Print version
```

Output of `lolfetch display help`:

```plaintext
Display information about a League of Legends account

Usage: lolfetch display [OPTIONS] <COMMAND>

Commands:
  ranked          Ranked information
  mastery         Mastery information
  recent-matches  Recent matches information
  custom          Custom information
  help            Print this message or the help of the given subcommand(s)

Options:
      --verbose
          Verbose mode

      --image <IMAGE>
          Image source for the ASCII art

          Possible values:
          - Default:  Default image display, based on the display type
          - rank:     Displays the rank of the player
          - champion: Displays the icon of a champion
          - profile:  Displays the icon of the summoner
          - Custom:   Displays a custom image
          
          [default: Default]

      --champion <CHAMPION> (this is only for the champion icon display)
          Name of the champion icon to display

      --custom-img-url <CUSTOM_IMG_URL> (this is only for the custom image display)
          Link to the custom image to display

      --no-save
          Don't save the cache to disk

  -h, --help
          Print help (see a summary with '-h')
```

## Disclaimer

Lolfetch isn't endorsed by Riot Games and doesn't reflect the views or opinions of Riot Games or anyone officially involved in producing or managing Riot Games properties. Riot Games, and all associated properties are trademarks or registered trademarks of Riot Games, Inc.
