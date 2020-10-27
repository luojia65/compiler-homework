/*
    S → a | λ | (T)
    T → ST'
    T' → ,ST' | ε
*/

struct SyntaxError {}

type Result<T> = core::result::Result<T, SyntaxError>;

fn s<I: Iterator<Item = char>>(iter: &mut I, sym: &mut Option<char>) -> Result<()> {
    println!("S; sym = {:?}", *sym);
    match sym {
        Some('a') | Some('λ') => *sym = iter.next(),
        Some('(') => {
            *sym = iter.next();
            t(iter, sym)?;
            if *sym == Some(')') {
                *sym = iter.next();
            } else {
                return Err(SyntaxError {})
            }
        },
        _ => return Err(SyntaxError {})
    }
    Ok(())
}

fn t<I: Iterator<Item = char>>(iter: &mut I, sym: &mut Option<char>) -> Result<()>  {
    println!("T; sym = {:?}", *sym);
    s(iter, sym)?;
    t1(iter, sym)?;
    Ok(())
}

fn t1<I: Iterator<Item = char>>(iter: &mut I, sym: &mut Option<char>) -> Result<()>  {
    println!("T'; sym = {:?}", *sym);
    if *sym == Some(',') {
        *sym = iter.next();
        s(iter, sym)?;
        t1(iter, sym)?;
        Ok(())
    } else if *sym == Some(')') {
        Ok(())
    } else {
        Err(SyntaxError {})
    }
}

fn work(string: &str) {
    let mut iter = string.chars();
    let mut sym = iter.next();
    let result = s(&mut iter, &mut sym);
    if let Some(sym) = sym {
        println!("Error: Unexpected tail: {}", sym);
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
