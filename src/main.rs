mod jokes;
use std::error::Error;

use argh::FromArgs;
use jokes::{Joke, JokeType, Jokes};

fn main() -> Result<(), Box<dyn Error>> {
    let args = argh::from_env::<Args>();
    let command = Command::parse(args);

    match command {
        Command::UpdateCommand => {
            Jokes::download_all_jokes();
        }
        Command::ListQuery => {
            println!("{}", JokeType::Dad);
            println!("{}", JokeType::General);
            println!("{}", JokeType::KnockKnock);
            println!("{}", JokeType::Programming);
        }
        Command::IdQuery(id) => {
            let all_jokes = Jokes::load_all_jokes();
            let joke = all_jokes.jokes.get(id as usize);
            if let Some(joke) = joke {
                joke.print_joke();
            }
        }
        Command::CategoryQuery(category, number) => {
            let all_jokes = Jokes::load_all_jokes();
            let jokes_count = number.unwrap_or(1);
            let category_jokes: Vec<&Joke> = all_jokes
                .jokes
                .iter()
                .filter(|joke| joke.joke_type == category)
                .collect();
            let random_jokes = take_random_jokes(category_jokes, jokes_count);
            random_jokes.iter().for_each(|joke| joke.print_joke());
        }
        Command::RandomQuery(number) => {
            let all_jokes = Jokes::load_all_jokes();
            let jokes_count = number.unwrap_or(1);
            let random_jokes = take_random_jokes(all_jokes.jokes.iter().collect(), jokes_count);
            random_jokes.iter().for_each(|joke| joke.print_joke());
        }
    }

    Ok(())
}

fn take_random_jokes(jokes: Vec<&Joke>, jokes_count: u32) -> Vec<&Joke> {
    if jokes_count > jokes.len() as u32 {
        return jokes;
    }

    struct WeightedJoke<'a> {
        joke: &'a Joke,
        weight: u32,
    }
    let mut sorted: Vec<WeightedJoke> = jokes
        .iter()
        .map(|joke| WeightedJoke {
            joke,
            weight: rand::random(),
        })
        .collect();
    sorted.sort_by_key(|j| j.weight);
    sorted
        .iter()
        .take(jokes_count as usize)
        .map(|j| j.joke)
        .collect()
}

/// Unofficial CLI to for the official JokeAPI (https://github.com/15Dkatz/official_joke_api)
#[derive(Debug, FromArgs)]
struct Args {
    /// download the latest jokes
    #[argh(switch)]
    update: bool,

    /// list all joke categories
    #[argh(switch, short = 'l')]
    list: bool,

    /// list joke from this category
    #[argh(option, short = 'c')]
    category: Option<JokeType>,

    /// list these many jokes
    #[argh(option, short = 'n')]
    number: Option<u32>,

    /// grab this joke by ID
    #[argh(option, short = 'i')]
    id: Option<u32>,
}

enum Command {
    UpdateCommand,
    ListQuery,
    IdQuery(u32),
    CategoryQuery(JokeType, Option<u32>),
    RandomQuery(Option<u32>),
}

impl Command {
    fn parse(args: Args) -> Command {
        if args.update == true {
            return Command::UpdateCommand;
        }

        if args.list == true {
            return Command::ListQuery;
        }

        if let Some(id) = args.id {
            return Command::IdQuery(id);
        }

        if let Some(category) = args.category {
            return Command::CategoryQuery(category, args.number);
        }

        Command::RandomQuery(args.number)
    }
}
