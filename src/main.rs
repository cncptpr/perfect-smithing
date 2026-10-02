use std::io;

use clap::Parser;

use shortest_sum::{
    CoreError, DEFAULT_MAX_POS, DEFAULT_STEPS, LastHits, Slot, floyd_warshall, load_cache, solve,
    store_cache,
};

#[derive(Parser)]
#[command(version)]
#[command(about = "Calculates shortest path to get from the start to the target position with a max position of max_pos.", long_about = None)]
struct Args {
    target: i64,
    #[arg(long, default_value_t = Args::default().start)]
    start: i64,
    #[arg(long, default_value_t = Args::default().max_pos)]
    max_pos: i64,
    // The literal has to stay in sync with DEFAULT_STEPS, clap needs a string here.
    #[arg(long, value_delimiter = ',', default_value = "-15,-6,-5,-3,2,7,13,16")]
    steps: Vec<i64>,
    #[arg(
        long,
        value_delimiter = ',',
        help = "Forced finishing hits in game order: last, second, third"
    )]
    last_hits: Vec<i64>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            start: 0,
            target: 0,
            max_pos: DEFAULT_MAX_POS,
            steps: DEFAULT_STEPS.to_vec(),
            last_hits: vec![],
        }
    }
}

fn fail(error: CoreError) -> ! {
    eprintln!("Error: {error}");
    std::process::exit(1);
}

fn main() -> io::Result<()> {
    let config = Args::parse();

    let last_hits = match LastHits::try_new(&config.last_hits, &config.steps) {
        Ok(last_hits) => last_hits,
        Err(error) => fail(error),
    };

    let result = match load_cache(config.max_pos as usize + 1, &config.steps) {
        Some(cache) => cache,
        None => {
            let result = floyd_warshall(config.max_pos + 1, config.steps.clone());
            _ = store_cache(&result)
                .inspect_err(|e| eprintln!("Error: Failed to store_cache: {e}"));
            result
        }
    };

    let solution = match solve(&result, config.start, config.target, &last_hits) {
        Ok(solution) => solution,
        Err(error) => fail(error),
    };

    let mut steps = solution.prefix_steps.clone();
    steps.extend(solution.suffix.iter().map(|(_, value)| value));
    steps.sort_unstable_by(|a, b| b.cmp(a));

    let distance = solution.path.len() - 1;

    // println!("\n{}\n{}\n", result.distance, result.predecessor);

    if config.start == 0 {
        println!(
            "It takes {} steps to get to {}: {:?}",
            distance, config.target, steps
        );
    } else {
        println!(
            "It takes {} steps to get from {} to {}: {:?}",
            distance, config.start, config.target, steps
        );
    }

    if !config.last_hits.is_empty() {
        let finishing = [Slot::Last, Slot::Second, Slot::Third]
            .into_iter()
            .zip(last_hits.slots())
            .filter_map(|(slot, value)| value.map(|value| format!("{slot}={value}")))
            .collect::<Vec<_>>()
            .join(", ");
        println!("Finishing with: {finishing}");
    }

    Ok(())
}

// impl Config {
//     fn try_from_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
//         let mut config = Config::default();
//         while let Some(arg) = args.next() {
//             match arg.as_str() {
//                 "in" => config.max_value = Self::parse_int(args.next(), "in", "max_value")?,
//                 "from" => config.start = Self::parse_int(args.next(), "from", "start")?,
//                 "to" => config.target = Self::parse_int(args.next(), "to", "target")?,
//                 "with" => config.steps = Self::parse_int_vec(&mut args.peekable(), "with", "step")?,
//                 _ => return Err(format!("Unkown keyword: {arg}")),
//             }
//         }
//         Ok(config)
//     }

//     fn parse_int(
//         value: Option<String>,
//         key: impl Into<String>,
//         name: impl Into<String>,
//     ) -> Result<i64, String> {
//         match value {
//             Some(v) => v
//                 .parse()
//                 .map_err(|e| format!("Failed to parse '{v}' into <{}>: {e}", name.into())),
//             None => Err(format!(
//                 "Expected <{}> after '{}', found nothing",
//                 name.into(),
//                 key.into(),
//             )),
//         }
//     }

//     fn parse_int_vec(
//         args: &mut Peekable<impl Iterator<Item = String>>,
//         key: impl Into<String>,
//         name: impl Into<String>,
//     ) -> Result<Vec<i64>, String> {
//         let first = args
//             .peek()
//             .ok_or(format!(
//                 "Expected at least one <{}> after '{}'",
//                 name.into(),
//                 name.into()
//             ))
//             .and_then(|arg| Self::parse_int(Some(*arg), key, name))?;
//         _ = args.next();

//         loop {}
//         match value {
//             Some(v) => v
//                 .parse()
//                 .map_err(|e| format!("Failed to parse '{v}' into <{}>: {e}", name.into())),
//             None => Err(format!(
//                 "Expected <{}> after '{}', found nothing",
//                 name.into(),
//                 key.into(),
//             )),
//         }
//     }
// }

// fn print_usage() {
//     print_usage_to(std::io::stdout());
// }

// fn print_usage_err() {
//     print_usage_to(std::io::stderr());
// }

// fn print_usage_to<T: Write>(mut writer: T) {
//     let config = Config::default();
//     write!(
//         writer,
//         "
// Usage:
//     shortest-sum to <target> [from <start>] [in <max_pos>] [with <step> [<step> ...]]

// Output:
//     The least amount of <step>s to get from the <start> position to the <target> position
//     with a max position of <max_pos>.

// Defaults:
//     start: {}
//     max_pos: {}
//     steps {:?}
// ",
//         config.start, config.max_pos, config.steps
//     );
// }
