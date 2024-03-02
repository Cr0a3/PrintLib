use PrintLib::logr;

fn main() {
    let logr: Logger = logr::def();
    logr.debug(String::from("DEBUG TEST!"));
    logr.info(String::from("INFO TEST!"));
    logr.warn(String::from("WARN TEST!"));
    logr.error(String::from("ERROR TEST!"));
}
