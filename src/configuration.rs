use std::env;
const DEFAULT_PORT: u16 = 3000;
const PORT_KEY: &str = "PORT";
const MACHINE_ID_KEY: &str = "MACHINE_ID";
pub struct Configuration {
    pub port: u16, //the port to bind the server to
    pub machine_id: u16, //the machine ID which *must* be unique per-machine
}

impl Configuration {
    pub fn from_env() -> Configuration {
        let port: u16 = match env::var(PORT_KEY) {
            Ok(val) => val.parse().unwrap(),
            Err(_) => DEFAULT_PORT,
        };

        let machine_id: u16 = match env::var(MACHINE_ID_KEY) {
            Ok(val) => val.parse().unwrap(),
            Err(_) => panic!("Must specify a MACHINE_ID"),
        };
        Configuration { port, machine_id }
    }
}
