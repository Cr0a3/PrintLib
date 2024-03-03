use PrintLib::colorize::Colorize;

fn main() {
    println!("{}", "i am blue".blue() );
    println!("{}", "i am custom!!!".color(123, 45, 167).bold().italic().underline() );
}
