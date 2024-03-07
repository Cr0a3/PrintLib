use PrintLib::arg::UsageFactory;

fn main() {
    let mut fab = UsageFactory::new("My cool application", "test");
    fab.add_opt("-v, --version", "prints version");
    fab.add_cmd("help\t\t", "example cmd");
    fab.print();
}
