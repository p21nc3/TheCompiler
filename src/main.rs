use std::fs;
use std::io::Write;
use std::path::Path;
use std::io::Read;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Incorrect usage. Correct usage is: hydro input.hy");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let mut contents = String::new();
    let mut input_file = fs::File::open(input_file).unwrap();
    input_file.read_to_string(&mut contents).unwrap();

    let tokens = tokenize(&contents);

    let tree = parse(&tokens);

    if tree.is_none() {
        eprintln!("No exit statement found");
        return;
    }

    let assembly_code = generate(&tree.unwrap().as_ref(), 50);


    let output_file_path = Path::new("out.asm");
    let mut output_file = fs::File::create(output_file_path).unwrap();
    output_file.write_all(assembly_code.as_bytes()).unwrap();
}

fn tokenize(contents: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    for mut c in contents.chars() {
        if c.is_alphabetic() {
            buf.push(c);
            while c.is_alphanumeric() {
                c = contents.chars().nth(buf.len()).unwrap();
                buf.push(c);
            }
            if KEYWORDS.contains(&buf.as_str()) {
                tokens.push(Token::from(Token::IntLit(buf.clone())));
                buf.clear();
                continue;
            }            
            else {
                eprintln!("You messed up!");
                std::process::exit(1);
            }
        }
        else if c.is_digit(1) {
            buf.push(c);
            while c.is_digit(1) {
                c = contents.chars().nth(buf.len()).unwrap();
                buf.push(c);
            }
            tokens.push(Token::IntLit(buf.clone()));
            buf.clear();
            continue;
        }
        else if c == ';' {
            tokens.push(Token::Semi);
            continue;
        }
        else if c.is_whitespace() {
            continue;
        }
        else {
            eprintln!("You messed up!");
            std::process::exit(1);
        }
    }
    buf.clear();
    return tokens;
}

const KEYWORDS: &[&str] = &["exit"];



#[derive(PartialEq)]
enum Token {
    Exit,
    IntLit(String),
    Semi,
}

struct NodeExit {}

impl AsRef<NodeExit> for NodeExit {
    fn as_ref(&self) -> &NodeExit {
        self
    }
}


fn parse(tokens: &[Token]) -> Option<NodeExit> {
    let mut exit_node = None;
    for token in tokens {
        if token == &Token::Exit {
            if exit_node.is_some() {
                eprintln!("Multiple exit statements found");
                return None;
            }
            exit_node = Some(NodeExit {});
        }
        else {
            eprintln!("Invalid token found");
            return None;
        }
    }
    exit_node
}

fn generate(_tree: &NodeExit, exit_status: i32) -> String {
    let _assembly_code = format!(
        "global _start\n_start:\n    mov rax, 60\n    mov rdi, {}\n    syscall\n",
        exit_status
    );
    String::new()
}

