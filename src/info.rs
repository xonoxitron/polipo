const NAME: &str = "polipo";
const DESCRIPTION: &str =
    "🐙 Minimal, elegant, fast, async Kraken exchange REST API client | Written in Rust";
const VERSION: &str = "0.2.5";
const AUTHOR: &str = "Matteo Pisani <matteo.pisani.91@gmail.com>";
const REPOSITORY: &str = "https://github.com/xonoxitron/polipo";

pub fn get_crate_info() -> String {
    format!(
        "Name: {}\r\nDescription: {}\r\nVersion: {}\r\nAuthor: {}\r\nRepository: {}",
        NAME, DESCRIPTION, VERSION, AUTHOR, REPOSITORY
    )
}
