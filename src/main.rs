use std::{fs::File, io::{stdout, Read, Write},};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();

    let delimiter = if args.len() > 1 {
        match args[1].parse::<String>() {
            Ok(delimiter) => delimiter,
            Err(_) => {
                println!("Error parsing delimiter: {}", args[1]);
                "\n".to_string()
            }
        }
    } else {
        "\n".to_string()
    };

    let content = read_files(&args[1..], &delimiter)?;

    match print_content(content, &delimiter) {
        Ok(_) => {},
        Err(err) => println!("Error printing content: {}", err),
    };

    Ok(())
}

fn read_files(file_paths: &[String], delimiter: &str) -> Result<String, std::io::Error> {
    let mut content = String::new();

    for file_path in file_paths {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(err) => {
                println!("Error opening file: {} ({})", file_path, err);
                continue;
            }
        };

        let mut buffer = [0; 1024];

        loop {
            let bytes_read = match file.read(&mut buffer) {
                Ok(bytes_read) => bytes_read,
                Err(err) => {
                    println!("Error reading from file: {} ({})", file_path, err);
                    break;
                }
            };

            if bytes_read == 0 {
                break;
            }

            content.push_str(&match std::str::from_utf8(&buffer[0..bytes_read]) {
                Ok(string) => string,
                Err(err) => {
                    eprintln!("Error decoding file: {} ({})", file_path, err);
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
                }
            });
        }

        if !content.is_empty() {
            content.push_str(delimiter);
        }
    }

    Ok(content)
}

fn print_content(content: String, _delimiter: &str) -> Result<(), std::io::Error> {
    let mut output = stdout();
    output.write_all(content.as_bytes())?;

    Ok(())
}
