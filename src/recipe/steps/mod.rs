use super::super::config::HostConfig;
use std::process::Command;
use std::io::{Error, ErrorKind};

pub mod core;
pub mod error;

#[derive(Debug)]
pub struct Context {
    pub config: HostConfig,
    pub release_path: String
}

impl Context {
    pub fn from_host_config(config: HostConfig) -> Result<Context, Error> {

        let output = Command::new("ssh")
            .args(&[&config.host, "date +'%Y%m%d%H%M%S'"])
            .output()?;

        if !output.status.success() {
            Err(Error::new(ErrorKind::Other, "Failed to compute current timestamp at the server"))
        } else {
            let release_timestamp = String::from_utf8_lossy(&output.stdout);
            let release_path = format!("{}/releases/{}", config.deploy_path, release_timestamp);

            Ok(Context { config, release_path })
        }
    }
}

pub trait Step {
    fn new(name: &'static str) -> Self where Self: Sized;
    fn execute(&self, context: &Context) -> Result<(), &str>;
    fn get_name(&self) -> &str;
}