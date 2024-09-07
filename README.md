# CLI for the Official Joke API

## Installation

### Build from source

- After cloning the repository locally, run the following from the repository root. You need to have Rust installed to compile and install the CLI
    - `cargo install --path .`

## Usage

- Run `joke` to get a random joke. The first run downloads all the jokes from the [Official Joke Api](https://github.com/15Dkatz/official_joke_api).
- Periodically run `joke --update` to update the list of jokes.
- Mix with [cowsay](https://en.wikipedia.org/wiki/Cowsay) for extra fun 
- `joke -c programming | cowsay -f tux`

```shell
 __________________________________
/ Knock-knock.                     \
|                                  |
\ A race condition. Who is there?  /
 ----------------------------------
   \
    \
        .--.
       |o_o |
       |:_/ |
      //   \ \
     (|     | )
    /'\_   _/`\
    \___)=(___/

```


