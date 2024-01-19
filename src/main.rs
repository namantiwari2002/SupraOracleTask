use std::fs;

mod client;
mod aggregator;

#[tokio::main]
async fn main()->Result<(),()> {

    let args: Vec<String> = std::env::args().collect();

    let mode = &args[1];
    match mode.as_str() {
        "--mode=cache" => {
            if let Some(times_arg) = args.iter().find(|arg| arg.starts_with("--times=")) {
                let times: u64 = times_arg.split('=').nth(1).unwrap().parse().unwrap_or_else(|_| {
                    eprintln!("Error: Invalid value for --times");
                    std::process::exit(1);
                });
                client::multiple_agents(times).await;
            } else {
                eprintln!("Error: --times is required for cache mode");
                std::process::exit(1);
            }
        }
        "--mode=read" => read_mode(),
        _ => {
            eprintln!("Error: Invalid mode. Use --mode=cache or --mode=read");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn read_mode() {
    let mut path = std::env::current_dir().unwrap().display().to_string();
    path+="/src/cache_data.txt";
     let contents = fs::read_to_string(path)
     .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

