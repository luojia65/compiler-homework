/*
    E → TE'
    E' → +TE' | ε
    T → FT'
    T' → *FT' | ε
    F → (E) | id
*/

struct SyntaxError {}

type Result<T> = core::result::Result<T, SyntaxError>;

#[derive(Debug, Eq, PartialEq)]
enum Word {
    Id(String),
    Add,
    Mul,
    LeftParent,
    RightParent,
    Other
}

struct Lexer<I: Iterator> {
    iter: core::iter::Peekable<I>
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Word;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.peek() {
            Some(&ch) if is_ident_char(ch)  => {
                let mut ans = String::new();
                while let Some(&ch) = self.iter.peek() {
                    if !is_ident_char(ch) {
                        break
                    }
                    ans.push(ch);
                    self.iter.next();
                }
                Some(Word::Id(ans))
            },
            Some('+') => { self.iter.next(); Some(Word::Add) },
            Some('*') => { self.iter.next(); Some(Word::Mul) },
            Some('(') => { self.iter.next(); Some(Word::LeftParent) },
            Some(')') => { self.iter.next(); Some(Word::RightParent) },
            None => None,
            _ => { self.iter.next(); Some(Word::Other) }
        }
    }
}

fn lexer(input: &str) -> Lexer<std::str::Chars> {
    Lexer {
        iter: input.chars().peekable() // LL(1)
    }
}

fn is_ident_char(ch: char) -> bool {
    ('a' ..= 'z').contains(&ch) || 
    ('A' ..= 'Z').contains(&ch) || 
    ('0' ..= '9').contains(&ch)
}

// E → TE'
fn e<I: Iterator<Item = Word>>(iter: &mut I, sym: &mut Option<Word>) -> Result<()> {
    println!("E; sym = {:?}", *sym);
    t(iter, sym)?;
    e1(iter, sym)?;
    Ok(())
}

// E' → +TE' | ε
fn e1<I: Iterator<Item = Word>>(iter: &mut I, sym: &mut Option<Word>) -> Result<()> {
    println!("E'; sym = {:?}", *sym);
    match *sym {
        Some(Word::Add) => {
            *sym = iter.next();
            t(iter, sym)?;
            e1(iter, sym)?;
            Ok(())
        },
        Some(Word::RightParent) => {
            Ok(())
        },
        None => Ok(()),
        _ => Err(SyntaxError {})
    }
}

// T → FT'
fn t<I: Iterator<Item = Word>>(iter: &mut I, sym: &mut Option<Word>) -> Result<()> {
    println!("T; sym = {:?}", *sym);
    f(iter, sym)?;
    t1(iter, sym)?;
    Ok(())
}

// T' → *FT' | ε
fn t1<I: Iterator<Item = Word>>(iter: &mut I, sym: &mut Option<Word>) -> Result<()> {
    println!("T'; sym = {:?}", *sym);
    match *sym {
        Some(Word::Mul) => {
            *sym = iter.next();
            f(iter, sym)?;
            t1(iter, sym)?;
            Ok(())
        },
        Some(Word::RightParent) | Some(Word::Add) => {
            Ok(())
        },
        None => Ok(()),
        _ => Err(SyntaxError {})
    }
}

// F → (E) | id
fn f<I: Iterator<Item = Word>>(iter: &mut I, sym: &mut Option<Word>) -> Result<()> {
    println!("F; sym = {:?}", *sym);
    match sym {
        Some(Word::Id(_)) => *sym = iter.next(),
        Some(Word::LeftParent) => {
            *sym = iter.next();
            e(iter, sym)?;
            if *sym == Some(Word::RightParent) {
                *sym = iter.next();
            } else {
                return Err(SyntaxError {})
            }
        },
        _ => return Err(SyntaxError {})
    }
    Ok(())
}

fn work(string: &str) {
    let mut iter = lexer(string);
    let mut sym = iter.next();
    let result = e(&mut iter, &mut sym);
    if let Some(sym) = sym {
        println!("Error: Unexpected tail: {:?}", sym);
        return;
    }
    match result {
        Ok(_) => println!("Matched!"),
        Err(_) => println!("Error!"),
    }
}

fn main() {
    let stdin = std::io::stdin();
    loop {
        let mut buf = String::new();
        let len = stdin.read_line(&mut buf).expect("stdin read");
        if len >= 1 {
            work(buf[..len].trim_end()); // trim the \r and \n
        }
    }
}
