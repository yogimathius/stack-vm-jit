use crate::vm::instruction::{Instruction, Opcode};
use crate::vm::types::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AssemblerError {
    InvalidOpcode(String),
    InvalidOperand(String),
    UnknownLabel(String),
    DuplicateLabel(String),
    ParseError(String),
    InvalidValue(String),
}

impl std::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssemblerError::InvalidOpcode(op) => write!(f, "Invalid opcode: {}", op),
            AssemblerError::InvalidOperand(op) => write!(f, "Invalid operand: {}", op),
            AssemblerError::UnknownLabel(label) => write!(f, "Unknown label: {}", label),
            AssemblerError::DuplicateLabel(label) => write!(f, "Duplicate label: {}", label),
            AssemblerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AssemblerError::InvalidValue(val) => write!(f, "Invalid value: {}", val),
        }
    }
}

impl std::error::Error for AssemblerError {}

pub struct Assembler {
    labels: HashMap<String, usize>,
    constants: Vec<Value>,
    constants_map: HashMap<String, usize>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
            constants: Vec::new(),
            constants_map: HashMap::new(),
        }
    }

    pub fn assemble(&mut self, source: &str) -> Result<(Vec<Instruction>, Vec<Value>), AssemblerError> {
        let lines: Vec<&str> = source.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with(';'))
            .collect();

        // First pass: collect labels and constants
        let mut instructions_without_labels = Vec::new();
        let mut instruction_index = 0;

        for line in &lines {
            if line.starts_with('.const') {
                self.parse_constant(line)?;
            } else if line.ends_with(':') {
                // Label
                let label = line.trim_end_matches(':').to_string();
                if self.labels.contains_key(&label) {
                    return Err(AssemblerError::DuplicateLabel(label));
                }
                self.labels.insert(label, instruction_index);
            } else {
                // Instruction - we'll parse it in the second pass
                instructions_without_labels.push(*line);
                instruction_index += 1;
            }
        }

        // Second pass: parse instructions with label resolution
        let mut instructions = Vec::new();
        for line in instructions_without_labels {
            let instruction = self.parse_instruction(line)?;
            instructions.push(instruction);
        }

        Ok((instructions, self.constants.clone()))
    }

    fn parse_constant(&mut self, line: &str) -> Result<(), AssemblerError> {
        // .const NAME VALUE
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(AssemblerError::ParseError(
                "Constant declaration must be: .const NAME VALUE".to_string()
            ));
        }

        let name = parts[1].to_string();
        let value = self.parse_value(parts[2])?;

        let index = self.constants.len();
        self.constants.push(value);
        self.constants_map.insert(name, index);

        Ok(())
    }

    fn parse_instruction(&self, line: &str) -> Result<Instruction, AssemblerError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Err(AssemblerError::ParseError("Empty instruction".to_string()));
        }

        let opcode_str = parts[0].to_uppercase();
        let opcode = self.parse_opcode(&opcode_str)?;

        let operand = if parts.len() > 1 {
            Some(self.parse_operand(parts[1])?)
        } else {
            None
        };

        Ok(Instruction::new(opcode, operand))
    }

    fn parse_opcode(&self, opcode_str: &str) -> Result<Opcode, AssemblerError> {
        match opcode_str {
            "PUSH" => Ok(Opcode::Push),
            "POP" => Ok(Opcode::Pop),
            "ADD" => Ok(Opcode::Add),
            "SUB" | "SUBTRACT" => Ok(Opcode::Subtract),
            "MUL" | "MULTIPLY" => Ok(Opcode::Multiply),
            "DIV" | "DIVIDE" => Ok(Opcode::Divide),
            "MOD" | "MODULO" => Ok(Opcode::Modulo),
            "AND" => Ok(Opcode::And),
            "OR" => Ok(Opcode::Or),
            "NOT" => Ok(Opcode::Not),
            "XOR" => Ok(Opcode::Xor),
            "SHL" | "SHIFT_LEFT" => Ok(Opcode::ShiftLeft),
            "SHR" | "SHIFT_RIGHT" => Ok(Opcode::ShiftRight),
            "EQ" | "EQUAL" => Ok(Opcode::Equal),
            "NE" | "NOT_EQUAL" => Ok(Opcode::NotEqual),
            "LT" | "LESS" => Ok(Opcode::Less),
            "LE" | "LESS_EQUAL" => Ok(Opcode::LessOrEqual),
            "GT" | "GREATER" => Ok(Opcode::Greater),
            "GE" | "GREATER_EQUAL" => Ok(Opcode::GreaterOrEqual),
            "JMP" | "JUMP" => Ok(Opcode::Jump),
            "JT" | "JUMP_TRUE" => Ok(Opcode::JumpIfTrue),
            "JF" | "JUMP_FALSE" => Ok(Opcode::JumpIfFalse),
            "CALL" => Ok(Opcode::Call),
            "RET" | "RETURN" => Ok(Opcode::Return),
            "LOAD" => Ok(Opcode::LoadLocal),
            "STORE" => Ok(Opcode::StoreLocal),
            "LOADC" | "LOAD_CONST" => Ok(Opcode::LoadConstant),
            "DUP" | "DUPLICATE" => Ok(Opcode::Duplicate),
            "SWAP" => Ok(Opcode::Swap),
            "NEW" | "NEW_OBJECT" => Ok(Opcode::NewObject),
            "GET_FIELD" => Ok(Opcode::GetField),
            "SET_FIELD" => Ok(Opcode::SetField),
            "NEW_ARRAY" => Ok(Opcode::NewArray),
            "GET_ARRAY" => Ok(Opcode::ArrayGet),
            "SET_ARRAY" => Ok(Opcode::ArraySet),
            "LEN" | "LENGTH" => Ok(Opcode::ArrayLength),
            "HALT" => Ok(Opcode::Halt),
            "NOP" | "NO_OP" => Ok(Opcode::NoOp),
            _ => Err(AssemblerError::InvalidOpcode(opcode_str.to_string())),
        }
    }

    fn parse_operand(&self, operand_str: &str) -> Result<Value, AssemblerError> {
        // Handle label references
        if let Some(&address) = self.labels.get(operand_str) {
            return Ok(Value::Integer(address as i64));
        }

        // Handle constant references
        if let Some(&index) = self.constants_map.get(operand_str) {
            return Ok(Value::Integer(index as i64));
        }

        // Handle direct values
        self.parse_value(operand_str)
    }

    fn parse_value(&self, value_str: &str) -> Result<Value, AssemblerError> {
        // Integer
        if let Ok(int_val) = value_str.parse::<i64>() {
            return Ok(Value::Integer(int_val));
        }

        // Float
        if let Ok(float_val) = value_str.parse::<f64>() {
            return Ok(Value::Float(float_val));
        }

        // Boolean
        match value_str.to_lowercase().as_str() {
            "true" => return Ok(Value::Boolean(true)),
            "false" => return Ok(Value::Boolean(false)),
            _ => {}
        }

        // String (enclosed in quotes)
        if value_str.starts_with('"') && value_str.ends_with('"') && value_str.len() >= 2 {
            let string_content = &value_str[1..value_str.len()-1];
            return Ok(Value::String(string_content.to_string()));
        }

        Err(AssemblerError::InvalidValue(value_str.to_string()))
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

// High-level language compiler for a simple stack-based language
pub struct SimpleCompiler {
    assembler: Assembler,
}

impl SimpleCompiler {
    pub fn new() -> Self {
        Self {
            assembler: Assembler::new(),
        }
    }

    pub fn compile_expression(&mut self, expr: &str) -> Result<(Vec<Instruction>, Vec<Value>), AssemblerError> {
        let assembly = self.expression_to_assembly(expr)?;
        self.assembler.assemble(&assembly)
    }

    fn expression_to_assembly(&self, expr: &str) -> Result<String, AssemblerError> {
        // Simple expression compiler for basic arithmetic
        // This is a very basic implementation - a full compiler would use proper parsing
        
        let tokens = self.tokenize(expr);
        let postfix = self.infix_to_postfix(tokens)?;
        
        let mut assembly = String::new();
        
        for token in postfix {
            match token.as_str() {
                "+" => assembly.push_str("ADD\n"),
                "-" => assembly.push_str("SUB\n"),
                "*" => assembly.push_str("MUL\n"),
                "/" => assembly.push_str("DIV\n"),
                "%" => assembly.push_str("MOD\n"),
                _ => {
                    if let Ok(_) = token.parse::<i64>() {
                        assembly.push_str(&format!("PUSH {}\n", token));
                    } else if let Ok(_) = token.parse::<f64>() {
                        assembly.push_str(&format!("PUSH {}\n", token));
                    } else {
                        return Err(AssemblerError::ParseError(format!("Unknown token: {}", token)));
                    }
                }
            }
        }
        
        assembly.push_str("HALT\n");
        Ok(assembly)
    }

    fn tokenize(&self, expr: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        
        for ch in expr.chars() {
            match ch {
                ' ' | '\t' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
                '+' | '-' | '*' | '/' | '%' | '(' | ')' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    tokens.push(ch.to_string());
                }
                _ => {
                    current_token.push(ch);
                }
            }
        }
        
        if !current_token.is_empty() {
            tokens.push(current_token);
        }
        
        tokens
    }

    fn infix_to_postfix(&self, tokens: Vec<String>) -> Result<Vec<String>, AssemblerError> {
        let mut output = Vec::new();
        let mut operators = Vec::new();
        
        for token in tokens {
            match token.as_str() {
                "(" => operators.push(token),
                ")" => {
                    while let Some(op) = operators.pop() {
                        if op == "(" {
                            break;
                        }
                        output.push(op);
                    }
                }
                "+" | "-" => {
                    while let Some(op) = operators.last() {
                        if op == "(" || self.precedence(op) < self.precedence(&token) {
                            break;
                        }
                        output.push(operators.pop().unwrap());
                    }
                    operators.push(token);
                }
                "*" | "/" | "%" => {
                    while let Some(op) = operators.last() {
                        if op == "(" || self.precedence(op) < self.precedence(&token) {
                            break;
                        }
                        output.push(operators.pop().unwrap());
                    }
                    operators.push(token);
                }
                _ => {
                    // Number or variable
                    output.push(token);
                }
            }
        }
        
        while let Some(op) = operators.pop() {
            output.push(op);
        }
        
        Ok(output)
    }

    fn precedence(&self, op: &str) -> i32 {
        match op {
            "+" | "-" => 1,
            "*" | "/" | "%" => 2,
            _ => 0,
        }
    }
}

impl Default for SimpleCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembler_basic() {
        let mut assembler = Assembler::new();
        let source = r#"
            PUSH 5
            PUSH 3
            ADD
            HALT
        "#;

        let result = assembler.assemble(source);
        assert!(result.is_ok());
        
        let (instructions, constants) = result.unwrap();
        assert_eq!(instructions.len(), 4);
        assert_eq!(constants.len(), 0);
    }

    #[test]
    fn test_assembler_with_labels() {
        let mut assembler = Assembler::new();
        let source = r#"
            PUSH 10
        loop:
            DUP
            PUSH 0
            GT
            JF end
            PUSH 1
            SUB
            JMP loop
        end:
            HALT
        "#;

        let result = assembler.assemble(source);
        assert!(result.is_ok());
        
        let (instructions, _) = result.unwrap();
        assert!(instructions.len() > 5);
    }

    #[test]
    fn test_assembler_with_constants() {
        let mut assembler = Assembler::new();
        let source = r#"
            .const MAX_VALUE 100
            .const MIN_VALUE 0
            
            PUSH MAX_VALUE
            PUSH MIN_VALUE
            ADD
            HALT
        "#;

        let result = assembler.assemble(source);
        assert!(result.is_ok());
        
        let (instructions, constants) = result.unwrap();
        assert_eq!(instructions.len(), 4);
        assert_eq!(constants.len(), 2);
    }

    #[test]
    fn test_simple_compiler() {
        let mut compiler = SimpleCompiler::new();
        let result = compiler.compile_expression("5 + 3 * 2");
        assert!(result.is_ok());
        
        let (instructions, constants) = result.unwrap();
        assert!(instructions.len() > 4); // Should have push, push, push, mul, add, halt
    }

    #[test]
    fn test_compiler_with_parentheses() {
        let mut compiler = SimpleCompiler::new();
        let result = compiler.compile_expression("(5 + 3) * 2");
        assert!(result.is_ok());
        
        let (instructions, _) = result.unwrap();
        assert!(instructions.len() > 4);
    }
}