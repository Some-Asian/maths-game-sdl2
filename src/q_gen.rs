use rand::prelude::*;
use rand::seq::SliceRandom;
use std::vec::Vec;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub enum Num_Type {
    Number(isize),
    Equation(Box<Equation>),
    Ratio((isize, isize)),
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum Ans_Type {
    Number(isize),
    Ratio((isize, isize))
}

#[derive(Debug, Clone)]
pub struct Equation {
    pub lhs: Num_Type,
    pub op: Operations,
    pub rhs: Num_Type,
}

impl Equation {
    pub const DIGIT_DIFFICULTIES: [f64; 10] = 
    [0.03, 0.06, 0.09, 0.09, 0.07, 0.1, 0.13, 0.11, 0.08, 0.01];

    pub fn gen_equation(rng: &mut ThreadRng, ballpark: Option<u32>, allowed_ops: Option<Vec<Operations>>) -> Self {
        let target_difficulty: u32 = if let Some(n) = ballpark {
            n
        } else {
            rng.gen_range(1..=5)
        };
        let mut possible_operators: Vec<Operations>;
        if let Some(v) = allowed_ops {
            possible_operators = v;
        }
        else {
            possible_operators = vec![Operations::Add, Operations::Subtract, Operations::Multiply];
            if target_difficulty >= 2 {
                possible_operators.push(Operations::Divide);
            }
            if target_difficulty >= 3 {
                possible_operators.append(&mut vec![Operations::Square, Operations::Simplify]);
            }
            if target_difficulty >= 4 {
                possible_operators.append(&mut vec![Operations::Sqrt, Operations::Cube])
            }
        }
        let op = possible_operators.choose(rng).unwrap().clone();
        let mut temp_rhs: Option<u32> = None;
        const TEN: u32 = 10;
        let bounds = vec![6, 12, 20, 50, 100];
        let lhs: Num_Type = match op {
            Operations::Add | Operations::Subtract => {
                Num_Type::Number(rng.gen_range((TEN.pow(target_difficulty - 1)..=(TEN.pow(target_difficulty)))) as isize)
            },
            Operations::Multiply | Operations::Square | Operations::Cube => {
                Num_Type::Number(rng.gen_range(1..=bounds[(target_difficulty - 1)as usize]) as isize)
            },
            Operations::Divide | Operations::Simplify => {
                let right_hand = rng.gen_range(1..=bounds[(target_difficulty - 1)as usize]);
                temp_rhs = Some(right_hand);
                Num_Type::Number((rng.gen_range(1..=bounds[(target_difficulty - 1)as usize]) * right_hand) as isize)
            },
            Operations::Sqrt => {
                let root: u32 = rng.gen_range(1..=bounds[(target_difficulty - 1)as usize]);
                temp_rhs = Some(root);
                Num_Type::Number(root.pow(2) as isize)
            },
            _ => todo!()
        };
        if let Operations::Simplify = op {
            let extra_num = rng.gen_range(1..bounds[(target_difficulty - 1)as usize]);
            temp_rhs = Some(temp_rhs.unwrap() * extra_num as u32);
        }
        let rhs = if let Some(n) = temp_rhs {
            Num_Type::Number(n as isize)
        } else { match (op, &lhs) {
            (Operations::Add | Operations::Subtract, &Num_Type::Number(n)) => {
                Num_Type::Number(rng.gen_range(1..=n) as isize)
            },
            (Operations::Multiply, _) => {
                Num_Type::Number(rng.gen_range(1..=bounds[(target_difficulty - 1)as usize]) as isize)
            },
            _ => {Num_Type::None}
        }};

        Equation {
            lhs, op, rhs
        }
    }

    pub fn pick_equation(rng: &mut ThreadRng, sample_size: u32, target_difficulty: f64, strict: bool, allowed_ops: Option<Vec<Operations>>) -> Self {
        let mut equation_sample: Vec<Equation> = vec![];
        let ballpark: Option<u32> = Some(target_difficulty.floor() as u32);
        for _ in 0..sample_size {
            equation_sample.push(Equation::gen_equation(rng, ballpark, allowed_ops.clone()))
        }
        let mut sample_closeness: Vec<(&Equation, f64)> = equation_sample.iter().map(|eq| {
            let est_diff = eq.est_difficulty();
            (eq, (est_diff - target_difficulty).abs())
        }).collect();
        if strict {
            sample_closeness.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            return (sample_closeness[0].0).clone()
        }
        else {
            let mut maximum_closeness = 0.0;
            for sample in &sample_closeness {
                if sample.1 > maximum_closeness {
                    maximum_closeness = sample.1;
                }
            }
            let sample_weighted: Vec<(&&Equation, f64)> = sample_closeness.iter().map(|(eq, closeness)| {
                (eq, (maximum_closeness - closeness + 1.0).powf(2.0))
            }).collect();

            return (sample_weighted.choose_weighted(rng, |item| item.1).unwrap().0).clone().clone()
        }
    }

    pub fn print(&self) {
        let right_hand = match self.rhs {
            Num_Type::Number(n) => format!("{}", n),
            _ => format!("n")
        };
        let left_hand = match self.lhs {
            Num_Type::Number(n) => format!("{}", n),
            Num_Type::Ratio((a,b)) => format!("{}/{}", a, b),
            _ => format!("n")
        };
        match self.op {
            Operations::Add => println!("{} + {}", left_hand, right_hand),
            Operations::Subtract => println!("{} - {}", left_hand, right_hand),
            Operations::Multiply => println!("{} x {}", left_hand, right_hand),
            Operations::Divide => println!("{} / {}", left_hand, right_hand),
            Operations::Simplify => println!("{} : {}", left_hand, right_hand),
            Operations::Square => println!("{}^2", left_hand),
            Operations::Cube => println!("{}^3", left_hand),
            Operations::Sqrt => println!("sqrt({})", left_hand)
        }
    }

    pub fn est_difficulty(&self) -> f64 {
        let lhs_num = match self.lhs {
            Num_Type::Number(n) => n,
            _ => todo!()
        };
        let lhs_string = format!("{}", lhs_num);
        
        let rhs_string = if let Num_Type::Number(n) = self.rhs {
            format!("{}", n)
        } else {
            String::from("0")
        };
        let rhs_num = if let Num_Type::Number(n) = self.rhs {
            n
        } else {
            0
        };
        let mut difficulty_score: f64 = 0.0;
        let mut num_list: Vec<&str> = lhs_string.split("").filter(|d| d != &"").collect();
        let mut right_list: Vec<&str> = rhs_string.split("").filter(|d| d != &"").collect();
        num_list.append(&mut right_list);
        for (index, number) in num_list.into_iter().enumerate() {
            let digit: usize = match number.parse::<usize>() {
                Ok(d) => d,
                Err(_) => panic!("Number parse did not work...")
            };
            difficulty_score += Equation::DIGIT_DIFFICULTIES[digit] * ((index + 1) as f64).sqrt()
        }

        difficulty_score += match self.op {
            Operations::Add => 0.45 + (lhs_string.len() + rhs_string.len()) as f64 * 0.25,
            Operations::Subtract => 0.5 + (lhs_string.len() as f64) * 0.25 + (rhs_string.len() as f64) * 0.3,
            Operations::Multiply => 0.8 + ((lhs_num as f64) * 0.09 + (rhs_num as f64) * 0.09).sqrt(),
            Operations::Divide => 0.8 + (lhs_string.len() as f64) * 0.45 + (lhs_string.len() - rhs_string.len()) as f64 * 0.65,
            Operations::Simplify => 0.8 + (lhs_string.len() as f64) * 0.45 + (lhs_string.len() as i32 - rhs_string.len() as i32).abs() as f64 * 0.65,
            Operations::Square => 0.85 + (lhs_num as f64) * 0.065,
            Operations::Cube => 1.05 + (lhs_num as f64) * 0.090,
            Operations::Sqrt => 1.95 + (lhs_string.len() as f64) * 0.75
        };

        difficulty_score
    }

    pub fn gcd(a: isize, b: isize) -> isize {
        let mut n: isize = a.abs();
        let mut m = b.abs();
        while m != 0 {
            if m < n {
                std::mem::swap(&mut m, &mut n)
            }
            m %= n;
        }
        n
    }
    pub fn simplify(ratio: Num_Type) -> Num_Type {
        if let Num_Type::Ratio((n,d)) = ratio {
            let gcd = Equation::gcd(n, d);
            if d/gcd == 1 {
                return Num_Type::Number(n/gcd)
            }
            return Num_Type::Ratio((n/gcd,d/gcd))
        } else {
            return ratio
        }
    }

    pub fn evaluate_equation(&self) -> Num_Type {
        let answer: Num_Type = match self.op {
            Operations::Add => match (&self.lhs, &self.rhs) {
                (&Num_Type::Number(a), &Num_Type::Number(b)) => Num_Type::Number(a + b),
                (&Num_Type::Ratio((a_n,a_d)), &Num_Type::Number(b)) => Num_Type::Ratio((a_n + a_d * b, a_d)),
                (&Num_Type::Number(a), &Num_Type::Ratio((b_n,b_d))) => Num_Type::Ratio((b_n + b_d * a, b_d)),
                (&Num_Type::Ratio((a_n,a_d)), &Num_Type::Ratio((b_n,b_d))) => Num_Type::Ratio((a_n * b_d + b_n * a_d, a_d * b_d)),
                _ => todo!()
            },
            Operations::Subtract => match (&self.lhs, &self.rhs) {
                (&Num_Type::Number(a), &Num_Type::Number(b)) => Num_Type::Number(a - b),
                (&Num_Type::Ratio((a_n,a_d)), &Num_Type::Number(b)) => Num_Type::Ratio((a_n - a_d * b, a_d)),
                (&Num_Type::Number(a), &Num_Type::Ratio((b_n,b_d))) => Num_Type::Ratio((b_n - b_d * a, b_d)),
                (&Num_Type::Ratio((a_n,a_d)), &Num_Type::Ratio((b_n,b_d))) => Num_Type::Ratio((a_n * b_d - b_n * a_d, a_d * b_d)),
                _ => todo!()
            },
            Operations::Multiply => match (&self.lhs, &self.rhs) {
                (&Num_Type::Number(a), &Num_Type::Number(b)) => Num_Type::Number(a * b),
                (&Num_Type::Ratio((a_n,a_d)), &Num_Type::Number(b)) => Num_Type::Ratio((a_n * b, a_d)),
                (&Num_Type::Number(a), &Num_Type::Ratio((b_n,b_d))) => Num_Type::Ratio((b_n * a, b_d)),
                (&Num_Type::Ratio((a_n,a_d)), &Num_Type::Ratio((b_n,b_d))) => Num_Type::Ratio((a_n * b_n, a_d * b_d)),
                _ => todo!()
            },
            Operations::Divide => match (&self.lhs, &self.rhs) {
                (&Num_Type::Number(a), &Num_Type::Number(b)) => Num_Type::Number(a/b),
                (&Num_Type::Ratio((a_n,a_d)), &Num_Type::Number(b)) => Num_Type::Ratio((a_n, a_d * b)),
                (&Num_Type::Number(a), &Num_Type::Ratio((b_n,b_d))) => Num_Type::Ratio((a * b_d, b_n)),
                (&Num_Type::Ratio((a_n,a_d)), &Num_Type::Ratio((b_n,b_d))) => Num_Type::Ratio((a_n * b_d, b_n * a_d)),
                _ => todo!()
            },
            Operations::Square => match &self.lhs {
                &Num_Type::Number(a) => Num_Type::Number(a * a),
                &Num_Type::Ratio((a_n, a_d)) => Num_Type::Ratio((a_n * a_n, a_d * a_d)),
                _ => todo!()
            },
            Operations::Cube => match &self.lhs {
                &Num_Type::Number(a) => Num_Type::Number(a * a * a),
                &Num_Type::Ratio((a_n, a_d)) => Num_Type::Ratio((a_n * a_n * a_n, a_d * a_d * a_d)),
                _ => todo!()
            },
            Operations::Sqrt => match (&self.lhs, &self.rhs) {
                (&Num_Type::Number(_), &Num_Type::Number(ans)) => Num_Type::Number(ans),
                (&Num_Type::Ratio(_), &Num_Type::Ratio((ans_n, ans_d))) => Num_Type::Ratio((ans_n, ans_d)),
                _ => todo!()
            },
            Operations::Simplify => match (&self.lhs, &self.rhs) {
                (&Num_Type::Number(a), &Num_Type::Number(b)) => Num_Type::Ratio((a, b)),
                _ => todo!()
            }
            _ => todo!()
        };
        Equation::simplify(answer)
    }
}
