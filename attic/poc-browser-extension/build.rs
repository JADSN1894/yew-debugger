use std::{env, error::Error, fmt::Display, fs, io::Error as StdIoError, path::Path};

type BuildRsResult<T> = Result<T, BuildRsError>;

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_TOML: &str = include_str!("./Cargo.toml");

#[derive(Debug)]
enum BuildRsError {
    StdIoError(StdIoError),
    Custom(String),
}

impl Display for BuildRsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildRsError::StdIoError(error) => write!(f, "{}", error),
            BuildRsError::Custom(error_string) => write!(f, "{}", error_string),
        }
    }
}

impl Error for BuildRsError {}

fn main() -> BuildRsResult<()> {
    let out_dir = env::var_os("OUT_DIR").ok_or(BuildRsError::Custom(
        "Cannot find OUT_DIR enviroment".into(),
    ))?;
    let dest_path = Path::new(&out_dir).join("hello.rs");

    let file_content = format!(
        r#"
        pub fn message() -> &'static str {{
            "{CARGO_PKG_VERSION}:{CARGO_PKG_NAME}"
        }}
    "#
    );

    fs::write(&dest_path, file_content).map_err(|error| BuildRsError::StdIoError(error))?;
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}

// fn main() -> BuildRsResult<()> {

//     Ok(())
// }
