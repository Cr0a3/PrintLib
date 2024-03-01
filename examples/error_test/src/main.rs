use PrintLib::Error::*;

fn main() {
    let mut fab: ErrorFactory = ErrorFactory::new(String::from("E_Test"), String::from("Example error message"));

    fab.add_arrow("file".to_string(), 1, 9);
    fab.add_code_line("Example Line".to_string(), true, 1, false);
    fab.add_where(9, 4, true, "\"Line\" first letter must not be capital".to_string());
    fab.print();
}
