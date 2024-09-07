use clap::ValueEnum;
use serde::Deserialize;
use std::fs::File;

pub struct Jokes {
    pub jokes: Vec<Joke>,
}

impl Jokes {
    pub fn load_all_jokes() -> Jokes {
        let f = File::open(get_jokes_cache_file());
        let all_jokes = match f {
            Ok(file) => {
                let jokes = serde_json::from_reader(file);
                if let Ok(jokes) = jokes {
                    jokes
                } else {
                    Vec::new()
                }
            }
            Err(_) => Jokes::download_all_jokes(),
        };

        Jokes { jokes: all_jokes }
    }

    pub fn download_all_jokes() -> Vec<Joke> {
        let response = reqwest::blocking::get(
            "https://raw.githubusercontent.com/15Dkatz/official_joke_api/master/jokes/index.json",
        );
        if let Ok(response) = response {
            let jokes = response.text();
            if let Ok(jokes) = jokes {
                let written = std::fs::write(get_jokes_cache_file(), &jokes);
                if let Err(e) = written {
                    eprintln!("Failed to write jokes to file: {}", e);
                }
                let jokes: serde_json::error::Result<Vec<Joke>> = serde_json::from_str(&jokes);
                if let Ok(jokes) = jokes {
                    return jokes;
                }
            }
        }

        Vec::new()
    }
}

fn get_jokes_cache_file() -> String {
    let home_dir = home::home_dir().unwrap();
    home_dir.join(".cache/jokes.json").to_str().unwrap().to_string()
}

#[derive(Deserialize)]
pub struct Joke {
    #[serde(rename = "setup")]
    pub setup: String,
    #[serde(rename = "punchline")]
    pub punchline: String,
    #[serde(rename = "type")]
    pub joke_type: JokeType,
}

impl Joke {
    pub fn print_joke(&self) {
        println!("{}\n\n", self.setup);
        println!("{}\n", self.punchline);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, ValueEnum)]
pub enum JokeType {
    #[serde(rename = "dad")]
    Dad,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "knock-knock")]
    KnockKnock,
    #[serde(rename = "programming")]
    Programming,
}

impl std::fmt::Display for JokeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JokeType::Dad => write!(f, "Dad"),
            JokeType::General => write!(f, "General"),
            JokeType::KnockKnock => write!(f, "Knock-Knock"),
            JokeType::Programming => write!(f, "Programming"),
        }
    }
}
