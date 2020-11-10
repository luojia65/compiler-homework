/*
    S' → S
    S → (S)
    S → id
    LR(0)
*/
use core::str::CharIndices;
use core::iter::Peekable;
use core::ops::Range;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Symbol<'a> {
    // S1,
    S,
    LeftParent,
    RightParent,
    Identifier(&'a str),
    Other,
    // None => End
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum SymbolId {
    // S1,
    S,
    LeftParent,
    RightParent,
    Identifier,
    Other,
}

impl<'a> Symbol<'a> {
    fn id(&self) -> SymbolId {
        match *self {
            // Symbol::S1 => SymbolId::S1,
            Symbol::S => SymbolId::S,
            Symbol::LeftParent => SymbolId::LeftParent,
            Symbol::RightParent => SymbolId::RightParent,
            Symbol::Identifier(_) => SymbolId::Identifier,
            Symbol::Other => SymbolId::Other,
        }
    }
}

struct Lex<'a> {
    string: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

fn lex(string: &str) -> Lex {
    Lex { 
        string, 
        iter: string.char_indices().peekable(),
    }
}

impl<'a> Iterator for Lex<'a> {
    type Item = (Range<usize>, Symbol<'a>);
    fn next(&mut self) -> Option<Self::Item> {
        let (span, symbol) = match self.iter.peek() {
            Some(&(idx, ch @ '(')) => {
                self.iter.next();
                (idx..idx+ch.len_utf8(), Symbol::LeftParent)
            },
            Some(&(idx, ch @ ')')) => {
                self.iter.next();
                (idx..idx+ch.len_utf8(), Symbol::RightParent)
            },
            Some(&(idx, ch)) if is_ident_char(ch) => {
                let mut end = idx;
                while let Some(&(idx, ch)) = self.iter.peek() {
                    if !is_ident_char(ch) {
                        end = idx;
                        break;
                    }
                    self.iter.next();
                };
                if self.iter.peek() == None {
                    end = self.string.len();
                }
                (idx..end, Symbol::Identifier(&self.string[idx..end]))
            },
            None => return None, // end
            Some(&(idx, ch)) => { 
                self.iter.next(); 
                (idx..idx+ch.len_utf8(), Symbol::Other)
            }
        };
        Some((span, symbol))
    }
}

fn is_ident_char(ch: char) -> bool {
    ('a' ..= 'z').contains(&ch) || 
    ('A' ..= 'Z').contains(&ch) || 
    ('0' ..= '9').contains(&ch)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum State {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Reduce {
    // Zero,
    One,
    Two,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Next {
    State(State),
    Reduce(Reduce),
    Accomplished
}

fn work(string: &str) {
    let mut action = HashMap::new();
    action.insert((State::Zero, Some(SymbolId::Identifier)), Next::State(State::Three));
    action.insert((State::Zero, Some(SymbolId::LeftParent)), Next::State(State::Two));
    action.insert((State::One, None), Next::Accomplished);
    action.insert((State::Two, Some(SymbolId::Identifier)), Next::State(State::Three));
    action.insert((State::Two, Some(SymbolId::LeftParent)), Next::State(State::Two));
    action.insert((State::Three, Some(SymbolId::Identifier)), Next::Reduce(Reduce::Two));
    action.insert((State::Three, Some(SymbolId::LeftParent)), Next::Reduce(Reduce::Two));
    action.insert((State::Three, Some(SymbolId::RightParent)), Next::Reduce(Reduce::Two));
    action.insert((State::Three, None), Next::Reduce(Reduce::Two));
    action.insert((State::Four, Some(SymbolId::RightParent)), Next::State(State::Five));
    action.insert((State::Five, Some(SymbolId::Identifier)), Next::Reduce(Reduce::One));
    action.insert((State::Five, Some(SymbolId::LeftParent)), Next::Reduce(Reduce::One));
    action.insert((State::Five, Some(SymbolId::RightParent)), Next::Reduce(Reduce::One));
    action.insert((State::Five, None), Next::Reduce(Reduce::One));
    let mut goto = HashMap::new();
    goto.insert((State::Zero, SymbolId::S), State::One);
    goto.insert((State::Two, SymbolId::S), State::Four);

    let mut lex = lex(string);

    let mut stack_state = Vec::new();
    stack_state.push(State::Zero);
    let mut stack_symbol = Vec::new();
    let mut cur = lex.next();
    loop {
        let top_state = *stack_state.last().expect("some top state");
        let cur_sym_id = cur.clone().map(|(_r, s)| s.id());
        let next_action = action.get(&(top_state, cur_sym_id));
        println!("stack: {:?}, sym: {:?}, cur: {:?}, action: {:?}", stack_state, stack_symbol, cur, next_action);
        match next_action {
            Some(Next::Accomplished) => break,
            Some(Next::State(state)) => {
                stack_state.push(*state);
                stack_symbol.push(cur);
                cur = lex.next();
            },
            Some(Next::Reduce(reduce)) => {
                let symbol = match reduce {
                    Reduce::One => { // S → (S)
                        for _ in 0..3 { stack_symbol.pop(); stack_state.pop(); }
                        Symbol::S
                    },
                    Reduce::Two => { // S → id
                        stack_symbol.pop(); stack_state.pop();
                        Symbol::S
                    },
                };
                stack_symbol.push(Some((0..0, symbol)));
                let top_state_2 = *stack_state.last().unwrap();
                let nxt_state = goto[&(top_state_2, symbol.id())];
                stack_state.push(nxt_state);
            },
            None => {
                println!("error!");
                break
            }
        }
    }
    // for pair in lex {
    //     println!("{:?}", pair);
    // }
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
