// Calculator REPL
//
// A feature-rich calculator with:
// - Operators: +, -, *, /, % (modulo), ** or ^ (power)
// - Functions: sin, cos, tan, asin, acos, atan, ln, log2, log10, exp, sqrt,
//             round, floor, ceil, abs
// - Constants: pi, e, phi (golden ratio), tau (2π), sqrt2, sqrt3
// - Memory: m0-m9 for storage, c0-c9 to clear, 'clear' for last result
// - Special: _ (last result), parentheses for grouping
// - Commands: help or ?, q/quit/exit
//
// Usage examples:
//   2 + 3 * 4        → 14
//   sin(pi/2)        → 1
//   _ * 2            → 2 (uses last result)
//   m0               → saves last result to m0
//   sqrt(m0)         → uses value from m0
//   round(pi * 100) / 100  → 3.14

use std::io::{self, Write};

#[derive(Debug)]
enum Command {
    Exit,
    Help,
    ClearResult,
    SaveMemory(usize),
    ClearMemory(usize),
    Evaluate(String),
}

#[derive(Debug)]
enum InputType {
    MemorySave(usize),
    MemoryClear(usize),
    Expression,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Operator(char),
    Power,
    LeftParen,
    RightParen,
    Function(String),
    Memory(usize),
    Constant(String),
    LastResult,
    EOF,
}

struct Calculator {
    memory: [f64; 10],
    last_result: f64,
}

impl Calculator {
    fn new() -> Self {
        Self { 
            memory: [0.0; 10],
            last_result: 0.0,
        }
    }


    fn tokenize(&self, input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.trim().chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' => {
                    chars.next();
                }
                '0'..='9' | '.' => {
                    let mut number = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_ascii_digit() || ch == '.' {
                            number.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(number.parse().map_err(|_| "Invalid number")?));
                }
                '+' | '-' | '/' | '%' => {
                    tokens.push(Token::Operator(chars.next().unwrap()));
                }
                '*' => {
                    chars.next();
                    match chars.peek() {
                        Some('*') => {
                            chars.next();
                            tokens.push(Token::Power);
                        }
                        _ => tokens.push(Token::Operator('*')),
                    }
                }
                '(' => {
                    chars.next();
                    tokens.push(Token::LeftParen);
                }
                ')' => {
                    chars.next();
                    tokens.push(Token::RightParen);
                }
                '_' => {
                    chars.next();
                    tokens.push(Token::LastResult);
                }
                '^' => {
                    chars.next();
                    tokens.push(Token::Power);
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut word = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphabetic() || ch.is_ascii_digit() {
                            word.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    // Check for memory locations, constants, and functions
                    match word.as_str() {
                        "pi" | "e" | "phi" | "tau" | "sqrt2" | "sqrt3" => tokens.push(Token::Constant(word)),
                        _ => match (word.starts_with('m'), word.len()) {
                            (true, 2) => {
                                if let Some(digit) = word.chars().nth(1).unwrap().to_digit(10) {
                                    if digit <= 9 {
                                        tokens.push(Token::Memory(digit as usize));
                                        continue;
                                    }
                                }
                                tokens.push(Token::Function(word));
                            }
                            _ => tokens.push(Token::Function(word)),
                        }
                    }
                }
                _ => {
                    // Skip invalid characters silently
                    chars.next();
                }
            }
        }
        
        tokens.push(Token::EOF);
        Ok(tokens)
    }

    fn parse_expression(&mut self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        self.parse_addition(tokens, pos)
    }

    fn parse_addition(&mut self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        let mut left = self.parse_multiplication(tokens, pos)?;

        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Operator('+') => {
                    *pos += 1;
                    let right = self.parse_multiplication(tokens, pos)?;
                    left += right;
                }
                Token::Operator('-') => {
                    *pos += 1;
                    let right = self.parse_multiplication(tokens, pos)?;
                    left -= right;
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_multiplication(&mut self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        let mut left = self.parse_power(tokens, pos)?;

        while *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Operator('*') => {
                    *pos += 1;
                    let right = self.parse_power(tokens, pos)?;
                    left *= right;
                }
                Token::Operator('/') => {
                    *pos += 1;
                    let right = self.parse_power(tokens, pos)?;
                    if right == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    left /= right;
                }
                Token::Operator('%') => {
                    *pos += 1;
                    let right = self.parse_power(tokens, pos)?;
                    if right == 0.0 {
                        return Err("Modulo by zero".to_string());
                    }
                    left %= right;
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_power(&mut self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        let left = self.parse_unary(tokens, pos)?;

        if *pos < tokens.len() && tokens[*pos] == Token::Power {
            *pos += 1;
            let right = self.parse_power(tokens, pos)?; // Right associative
            return Ok(left.powf(right));
        }

        Ok(left)
    }

    fn parse_unary(&mut self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        if *pos < tokens.len() {
            match &tokens[*pos] {
                Token::Operator('-') => {
                    *pos += 1;
                    return Ok(-self.parse_unary(tokens, pos)?);
                }
                Token::Operator('+') => {
                    *pos += 1;
                    return Ok(self.parse_unary(tokens, pos)?);
                }
                _ => {}
            }
        }

        self.parse_factor(tokens, pos)
    }

    fn parse_factor(&mut self, tokens: &[Token], pos: &mut usize) -> Result<f64, String> {
        if *pos >= tokens.len() {
            return Err("Unexpected end of expression".to_string());
        }

        match &tokens[*pos] {
            Token::Number(n) => {
                *pos += 1;
                Ok(*n)
            }
            Token::LeftParen => {
                *pos += 1;
                let result = self.parse_addition(tokens, pos)?;
                if *pos >= tokens.len() || tokens[*pos] != Token::RightParen {
                    return Err("Expected closing parenthesis".to_string());
                }
                *pos += 1;
                Ok(result)
            }
            Token::Function(name) => {
                *pos += 1;
                
                // Check if it's a known function first
                let is_known_function = matches!(name.as_str(), 
                    "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | 
                    "ln" | "log2" | "log10" | "exp" | "sqrt" |
                    "round" | "floor" | "ceil" | "abs"
                );
                
                if !is_known_function {
                    return Err(format!("Unknown function '{}'. Available functions: sin, cos, tan, asin, acos, atan, ln, log2, log10, exp, sqrt, round, floor, ceil, abs", name));
                }
                
                if *pos >= tokens.len() || tokens[*pos] != Token::LeftParen {
                    return Err(format!("Function '{}' requires parentheses: {}(...)", name, name));
                }
                *pos += 1;
                let arg = self.parse_addition(tokens, pos)?;
                if *pos >= tokens.len() || tokens[*pos] != Token::RightParen {
                    return Err("Expected closing parenthesis".to_string());
                }
                *pos += 1;

                match name.as_str() {
                    "sin" => Ok(arg.sin()),
                    "cos" => Ok(arg.cos()),
                    "tan" => Ok(arg.tan()),
                    "asin" => {
                        if arg < -1.0 || arg > 1.0 {
                            return Err("asin requires argument between -1 and 1".to_string());
                        }
                        Ok(arg.asin())
                    }
                    "acos" => {
                        if arg < -1.0 || arg > 1.0 {
                            return Err("acos requires argument between -1 and 1".to_string());
                        }
                        Ok(arg.acos())
                    }
                    "atan" => Ok(arg.atan()),
                    "ln" => {
                        if arg <= 0.0 {
                            return Err("ln requires positive argument".to_string());
                        }
                        Ok(arg.ln())
                    }
                    "log2" => {
                        if arg <= 0.0 {
                            return Err("log2 requires positive argument".to_string());
                        }
                        Ok(arg.log2())
                    }
                    "log10" => {
                        if arg <= 0.0 {
                            return Err("log10 requires positive argument".to_string());
                        }
                        Ok(arg.log10())
                    }
                    "exp" => Ok(arg.exp()),
                    "sqrt" => {
                        if arg < 0.0 {
                            return Err("sqrt requires non-negative argument".to_string());
                        }
                        Ok(arg.sqrt())
                    }
                    "round" => Ok(arg.round()),
                    "floor" => Ok(arg.floor()),
                    "ceil" => Ok(arg.ceil()),
                    "abs" => Ok(arg.abs()),
                    _ => unreachable!(), // Already checked above
                }
            }
            Token::Memory(idx) => {
                *pos += 1;
                Ok(self.memory[*idx])
            }
            Token::Constant(name) => {
                *pos += 1;
                match name.as_str() {
                    "pi" => Ok(std::f64::consts::PI),
                    "e" => Ok(std::f64::consts::E),
                    "phi" => Ok((1.0 + 5.0_f64.sqrt()) / 2.0), // Golden ratio
                    "tau" => Ok(2.0 * std::f64::consts::PI),   // 2π
                    "sqrt2" => Ok(std::f64::consts::SQRT_2),
                    "sqrt3" => Ok(3.0_f64.sqrt()),
                    _ => Err(format!("Unknown constant: {}", name)),
                }
            }
            Token::LastResult => {
                *pos += 1;
                Ok(self.last_result)
            }
            _ => Err("Expected number, function, constant, memory location, _, or opening parenthesis".to_string()),
        }
    }

    fn is_memory_save(&self, input: &str) -> Option<usize> {
        let input = input.trim();
        if input.starts_with('m') && input.len() == 2 {
            if let Some(digit) = input.chars().nth(1).unwrap().to_digit(10) {
                if digit <= 9 {
                    return Some(digit as usize);
                }
            }
        }
        None
    }

    fn is_memory_clear(&self, input: &str) -> Option<usize> {
        let input = input.trim();
        if input.starts_with('c') && input.len() == 2 {
            if let Some(digit) = input.chars().nth(1).unwrap().to_digit(10) {
                if digit <= 9 {
                    return Some(digit as usize);
                }
            }
        }
        None
    }


    fn classify_input(&self, input: &str) -> InputType {
        if let Some(idx) = self.is_memory_save(input) {
            InputType::MemorySave(idx)
        } else if let Some(idx) = self.is_memory_clear(input) {
            InputType::MemoryClear(idx)
        } else {
            InputType::Expression
        }
    }

    fn parse_command(&self, input: &str) -> Command {
        let input = input.trim();
        
        match input {
            "q" | "quit" | "exit" => Command::Exit,
            "?" | "help" => Command::Help,
            "clear" => Command::ClearResult,
            _ => match self.classify_input(input) {
                InputType::MemorySave(idx) => Command::SaveMemory(idx),
                InputType::MemoryClear(idx) => Command::ClearMemory(idx),
                InputType::Expression => Command::Evaluate(input.to_string()),
            }
        }
    }

    fn print_help(&self) {
        println!("Calculator REPL");
        println!("Supported operators: +, -, *, /, %, ** (or ^)");
        println!("Supported functions: sin, cos, tan, asin, acos, atan, ln, log2, log10, exp, sqrt");
        println!("                    round, floor, ceil, abs");
        println!("Constants: pi, e, phi, tau, sqrt2, sqrt3");
        println!("Use '_' to reference the last result");
        println!("Memory locations: m0 through m9");
        println!("  - Use 'm0' on a line by itself to save last result to m0");
        println!("  - Use 'm0' in expressions to recall value from m0");
        println!("  - Use 'c0' to clear memory location m0, 'clear' to clear last result");
        println!("Type 'q', 'quit', or 'exit' to exit");
    }

    fn evaluate(&mut self, input: &str) -> Result<f64, String> {
        // Check if it's a memory save command (just m0, m1, etc.)
        if let Some(mem_idx) = self.is_memory_save(input) {
            self.memory[mem_idx] = self.last_result;
            return Ok(self.last_result);
        }

        let tokens = self.tokenize(input)?;
        let mut pos = 0;
        let result = self.parse_expression(&tokens, &mut pos)?;
        
        if pos < tokens.len() - 1 { // -1 because of EOF token
            return Err("Unexpected tokens at end of expression".to_string());
        }
        
        self.last_result = result;
        Ok(result)
    }

    fn run(&mut self) {
        self.print_help();
        println!();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Error reading input");
                continue;
            }

            let input = input.trim();
            
            if input.is_empty() {
                continue;
            }
            
            match self.parse_command(input) {
                Command::Exit => {
                    println!("Goodbye!");
                    break;
                }
                Command::Help => {
                    self.print_help();
                }
                Command::ClearResult => {
                    self.last_result = 0.0;
                    println!("Cleared last result");
                }
                Command::SaveMemory(idx) => {
                    println!("Saved {} to m{}", self.last_result, idx);
                    self.memory[idx] = self.last_result;
                }
                Command::ClearMemory(idx) => {
                    self.memory[idx] = 0.0;
                    println!("Cleared m{}", idx);
                }
                Command::Evaluate(expr) => {
                    match self.evaluate(&expr) {
                        Ok(result) => println!("{}", result),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
        }
    }
}

fn main() {
    let mut calc = Calculator::new();
    calc.run();
}