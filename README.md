# CLI for the Official Joke API

## Installation

### Build from source

`cargo install --git https://github.com/cool-mist/joke-cli`

## Usage

- Run `joke` to get a random joke.
    - The first run downloads all the jokes from the [Official Joke Api](https://github.com/15Dkatz/official_joke_api).
    - Periodically run `joke --update` to update the list of jokes.
- Run `joke --help` for other options.
- Mix with [cowsay](https://en.wikipedia.org/wiki/Cowsay) and add to your shell profile for extra fun.
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


