use rand::prelude::*;
use rand::seq::SliceRandom;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide,
    Simplify,
    Square,
    Cube,
    Sqrt,
}

#[derive(Debug)]
pub enum Rhs_Type {
    Number(isize),
    Equation(Box<Equation>),
    None,
}

#[derive(Debug)]
pub enum Ans_Type {
    Number(isize),
    Ratio((isize, isize))
}

#[derive(Debug)]
pub struct Equation {
    pub lhs: isize,
    pub op: Operations,
    pub rhs: Rhs_Type,
}

impl Equation {
    pub const DIGIT_DIFFICULTIES: [f32; 10] = 
    [0.03, 0.06, 0.09, 0.09, 0.07, 0.1, 0.13, 0.11, 0.08, 0.01];

    pub fn gen_equation(rng: &mut ThreadRng, ballpark: Option<u32>) -> Self {
        let target_difficulty: u32 = if let Some(n) = ballpark {
            n
        } else {
            rng.gen_range(1..=5)
        };

        let mut possible_operators = vec![Operations::Add, Operations::Subtract, Operations::Multiply];
        if target_difficulty >= 2 {
            possible_operators.push(Operations::Divide);
        }
        if target_difficulty >= 3 {
            possible_operators.append(&mut vec![Operations::Square, Operations::Simplify]);
        }
        if target_difficulty >= 4 {
            possible_operators.append(&mut vec![Operations::Sqrt, Operations::Cube])
        }

        let op = possible_operators.choose(rng).unwrap().clone();
        let mut temp_rhs: Option<u32> = None;
        const TEN: u32 = 10;
        let bounds = vec![6, 12, 20, 50, 100];
        let lhs: isize = match op {
            Operations::Add | Operations::Subtract => {
                rng.gen_range((TEN.pow(target_difficulty - 1)..=(TEN.pow(target_difficulty))))
            },
            Operations::Multiply | Operations::Square | Operations::Cube => {
                rng.gen_range(1..=bounds[(target_difficulty - 1)as usize])
            },
            Operations::Divide | Operations::Simplify => {
                let right_hand = rng.gen_range(1..=bounds[(target_difficulty - 1)as usize]);
                temp_rhs = Some(right_hand);
                rng.gen_range(1..=bounds[(target_difficulty - 1)as usize]) * right_hand
            },
            Operations::Sqrt => {
                let root: u32 = rng.gen_range(1..=bounds[(target_difficulty - 1)as usize]);
                root.pow(2)
            }
        } as isize;
        let rhs = if let Some(n) = temp_rhs {
            Rhs_Type::Number(n as isize)
        } else { match op {
            Operations::Add | Operations::Subtract => {
                Rhs_Type::Number(rng.gen_range(1..=lhs) as isize)
            },
            Operations::Multiply => {
                Rhs_Type::Number(rng.gen_range(1..=bounds[(target_difficulty - 1)as usize]) as isize)
            },
            _ => {Rhs_Type::None}
        }};

        Equation {
            lhs, op, rhs
        }
    }

    pub fn print(&self) {
        let right_hand = match self.rhs {
            Rhs_Type::Number(n) => format!("{}", n),
            _ => format!("n")
        };
        match self.op {
            Operations::Add => println!("{} + {}", self.lhs, right_hand),
            Operations::Subtract => println!("{} - {}", self.lhs, right_hand),
            Operations::Multiply => println!("{} x {}", self.lhs, right_hand),
            Operations::Divide => println!("{} / {}", self.lhs, right_hand),
            Operations::Simplify => println!("{} : {}", self.lhs, right_hand),
            Operations::Square => println!("{}^2", self.lhs),
            Operations::Cube => println!("{}^3", self.lhs),
            Operations::Sqrt => println!("sqrt({})", self.lhs)
        }
    }
}
