use PrintLib::Logger::Logger;

fn main() {
    let logr: Logger = Logger::new();
    logr.debug(String::from("DEBUG TEST!"));
    logr.info(String::from("INFO TEST!"));
    logr.warn(String::from("WARN TEST!"));
    logr.error(String::from("ERROR TEST!"));
}
