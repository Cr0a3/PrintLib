use PrintLib::colorize::ColorEncoder;

fn main() {
    println!("{}", ColorEncoder::encode("<red>r<green>g<blue>b <white>is <magenta>cool<black>!"));
    println!("{}", ColorEncoder::encode("<&e8b017>i am custom color"));
}
