use std::vec;
use crate::colorize::Colorize;

pub struct UsageFactory {
    rn: String,
    name: String,
    options: Vec<Opt>,
    cmds: Vec<Opt>,
}

#[derive(Clone)]
struct Opt {
    name: String,
    does: String,
}

impl UsageFactory {
    pub fn new(name: &str, app_name: &str) -> Self {
        Self {
            rn: app_name.into(),
            name: name.into(),
            options: vec![],
            cmds: vec![],
        }
    }

    pub fn add_cmd(&mut self, name: &str, description: &str) {
        self.cmds.push(Opt {name: name.to_string(), does: description.to_string()});
    }

    pub fn add_opt(&mut self, name: &str, description: &str) {
        self.options.push(Opt {name: name.to_string(), does: description.to_string()});
    }

    pub fn print(&mut self) {
        //print
        println!("{}", self.name.color(0, 42, 71).bold());
        println!();
        println!("{} {}{}", "Usage:".underline(), self.rn.color(0, 42, 71).bold(),  " [OPTIONS] [CMD]".color(0, 42, 71).bold());
        println!();
        println!("{}", "Options:");
        for opt in self.options.clone().into_iter() {
            println!("   {}\t {} {}", opt.name.color(59, 4, 105), "|".color(227, 173, 25), opt.does);
        }

        println!("{}", "Common commands:");
        for cmd in self.cmds.clone().into_iter() {
            println!("   {}\t {} {}", cmd.name.color(59, 4, 105), "|".color(227, 173, 25), cmd.does);
        }

    }
}
