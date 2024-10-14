use spdx::LicenseItem;
use url::Url;

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

fn main() {
    let url = Url::parse("https://www.rust-lang.org");
}