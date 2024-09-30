use spdx::LicenseItem;

struct Font {
    id: String,
    name: String,
    homepage: Url,
    license: LicenseItem,
    versions: Vec<Version>,
    foundry: Foundry,
}

struct Version {
    version: String,
    
}

struct Foundry {}

struct Url(String);

impl Url {
    fn new(s: String) -> Url {
        Url(s)
    }
}

fn main() {
    let url = Url::new("https://www.rust-lang.org".to_string());
}