extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session};

fn main() {
    let mut event_handler = EventHandler::new();

    event_handler.recv();
}

struct EventHandler {
    nvim: Neovim,
    calculator: Calculator,
}

impl EventHandler {
    fn new() -> EventHandler {
        let mut session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let calculator = Calculator::new();

        EventHandler { nvim, calculator }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, values) in receiver {
            match Messages::from(event) {
                Messages::Add => {
                    let nums = values
                        .iter()
                        .map(|v| v.as_i64().unwrap())
                        .collect::<Vec<i64>>();

                    let sum = self.calculator.add(nums);

                    self.nvim
                        .command(&format!("echo \"Sum: {} \"", sum.to_string()))
                        .unwrap();
                }
                Messages::Minus => {
                    let mut nums = values.iter();
                    let p = nums.next().unwrap().as_i64().unwrap();
                    let q = nums.next().unwrap().as_i64().unwrap();

                    let minus = self.calculator.minus(p, q);

                    self.nvim
                        .command(&format!("echo \"Minus: {} \"", minus.to_string()))
                        .unwrap();
                }
                Messages::Multiply => {
                    let mut nums = values.iter();
                    let p = nums.next().unwrap().as_i64().unwrap();
                    let q = nums.next().unwrap().as_i64().unwrap();

                    let product = self.calculator.muliply(p, q);

                    self.nvim
                        .command(&format!("echo \"Product: {} \"", product.to_string()))
                        .unwrap();
                }
                Messages::Unknown(uevent) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", uevent))
                        .unwrap();
                    // unknown
                }
            }
        }
    }
}

struct Calculator;

impl Calculator {
    fn new() -> Calculator {
        Calculator {}
    }

    fn add(&self, nums: Vec<i64>) -> i64 {
        nums.iter().sum::<i64>()
    }

    fn minus(&self, p: i64, q: i64) -> i64 {
        p - q
    }

    fn muliply(&self, p: i64, q: i64) -> i64 {
        p * q
    }
}

enum Messages {
    Add,
    Minus,
    Multiply,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "add" => Messages::Add,
            "minus" => Messages::Minus,
            "multiply" => Messages::Multiply,
            _ => Messages::Unknown(event),
        }
    }
}
