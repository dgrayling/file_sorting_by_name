use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a file path as the only program argument");
        return;
    }

    let file_path = &args[1];
    println!("File path: {}", file_path); // Output the argument to the console

    let read_directory = fs::read_dir(file_path);

    if let Err(read_directory_error) = read_directory {
        println!("Failed to read directory: {}", read_directory_error);
        return;
    }

    let (file_map, substrings_map) = process_directory(file_path);

    let clusters = generate_clusters(&file_map, &substrings_map);

    for (substring, files) in &clusters {
        if files.len() > 1 {
            println!("\x1b[32mSubstring: {}\x1b[0m", substring);
            println!("\x1b[31mFiles: {:?}\x1b[0m", files);
            println!(
                "Absolute paths: {:?}",
                files
                    .iter()
                    .map(|file| file_map.get(file).unwrap())
                    .collect::<Vec<&String>>()
            );
        }
    }
}

fn process_directory(file_path: &str) -> (HashMap<String, String>, HashMap<String, Vec<String>>) {
    let mut file_map: HashMap<String, String> = HashMap::new(); // Create a HashMap to store file names and their absolute directories
    let mut substrings_map: HashMap<String, Vec<String>> = HashMap::new(); // Create a HashMap to store file names and their corresponding substrings

    let read_directory = fs::read_dir(file_path);

    if let Ok(entries) = read_directory {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    let absolute_path = entry
                        .path()
                        .canonicalize()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(); // Get the absolute path of the file
                    file_map.insert(file_name.to_string(), absolute_path); // Insert the file name and its absolute path into the HashMap

                    let substrings = generate_substrings(&file_name);
                    substrings_map.insert(file_name.to_string(), substrings); // Insert the file name and its corresponding substrings into the HashMap
                }
            }
        }
    }

    (file_map, substrings_map)
}

fn generate_substrings(s: &str) -> Vec<String> {
    let mut substrings = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 0..len {
        for j in i..len {
            substrings.push(chars[i..=j].iter().collect());
        }
    }
    substrings
}

fn generate_clusters(
    file_map: &HashMap<String, String>,
    substrings_map: &HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    let mut clusters: HashMap<String, Vec<String>> = HashMap::new(); // Create a HashMap to store clusters of files that share substrings
    for (file_name, substrings) in substrings_map {
        for substring in substrings {
            if let Some(files) = clusters.get_mut(substring) {
                files.push(file_name.to_string());
            } else {
                clusters.insert(substring.to_string(), vec![file_name.to_string()]);
            }
        }
    }
    clusters
}
