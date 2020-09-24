use clap::{load_yaml, App};
use edit_distance::edit_distance;
use itertools::Itertools;

use std::{cmp::max, fmt, fs, io, path::Path};

fn main() -> io::Result<()> {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let eol = matches.value_of_lossy("linesep").unwrap();
    let eoc = matches.value_of_lossy("columnsep").unwrap();
    let relative = matches.is_present("relative");
    let trim_whitespaces = matches.is_present("trim-whitespaces");
    let mut strings: Vec<(Option<String>, String)> = matches
        .values_of_lossy("string")
        .unwrap_or_default()
        .drain(..)
        .enumerate()
        .map(|(nr, s)| (Some(format!("<string{}>", nr)), s))
        .collect();
    if let Some(file) = matches.value_of_lossy("from-file") {
        let mut filenames = read_filenames_from_file(file.to_string())?;
        let additional_strings = filenames
            .drain(..)
            .map(|p| (Some(p.clone()), fs::read_to_string(&p)))
            .map(|(p, res)| (p, notify_err(res)))
            .filter_map(|(p, opt)| opt.map(|s| (p, s)));
        strings.extend(additional_strings)
    }
    if let Some(mut files) = matches.values_of_lossy("FILE") {
        let additional_strings = files
            .drain(..)
            .map(|p| (Some(p.clone()), fs::read_to_string(&p)))
            .map(|(p, res)| (p, notify_err(res)))
            .filter_map(|(p, opt)| opt.map(|s| (p, s)));
        strings.extend(additional_strings)
    }

    for tuple in strings.iter().combinations(2) {
        // Extract the combination
        let (name1, s1) = tuple[0];
        let (name2, s2) = tuple[1];
        // Unwrap names
        let name1 = name1.to_owned().unwrap_or("NONE".into());
        let name2 = name2.to_owned().unwrap_or("NONE".into());
        // Trim whitespaces, if enabled
        let s1 = if trim_whitespaces { s1.trim() } else { s1 };
        let s2 = if trim_whitespaces { s2.trim() } else { s2 };
        // Calculate the edit distance
        // Make it relative, if enabled
        let distance = if relative {
            let dist = edit_distance(&s1, &s2) as f32;
            let len = max(s1.len(), s2.len()) as f32;
            dist / len
        } else {
            edit_distance(&s1, &s2) as f32
        };
        print!("{4}{0}{2}{0}{3}{1}", eoc, eol, name1, name2, distance)
    }

    Ok(())
}

fn read_filenames_from_file<P>(path: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(path)?;
    let lines = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_owned());
    Ok(lines.collect())
}

fn notify_err<T, E>(result: Result<T, E>) -> Option<T>
where
    E: fmt::Debug,
{
    match result {
        Ok(inner) => Some(inner),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            None
        }
    }
}
