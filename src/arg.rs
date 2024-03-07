use std::{cmp::max, vec};
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

fn align(mut s1: String, len: usize) -> String {
    let len1 = s1.len();
    
    if len1 < len {
        s1.push_str(&" ".repeat(len - len1));
    }
    
    s1
}

fn longest_string(strings: &Vec<Opt>) -> Option<String> {
    if strings.is_empty() {
        return None;
    }

    let mut longest = &strings[0];

    for s in strings.iter() {
        if s.name.len() > longest.name.len() {
            longest = s;
        }
    }

    Some(longest.name.clone())
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
        // aling
        let long_cmd = longest_string(&self.cmds).expect("Error while geting longest").len();
        let long_opt = longest_string(&self.options).expect("Error while geting longest").len();
        let longest = max(long_cmd, long_opt);

        let mut i = 1;
        while i < self.cmds.len() {
            self.cmds[i].name = align(self.cmds[i].name.clone(), longest);
            i += 1;
        }

        let mut i = 1;
        while i < self.options.len() {
            self.cmds[i].name = align(self.cmds[i].name.clone(), longest);
            i += 1;
        }

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