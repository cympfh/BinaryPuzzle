use std::io::{ self, Write };
use std::str::FromStr;
use std::collections::{ VecDeque, HashSet };

macro_rules! trace {
    ($var:expr) => ({
        let _ = writeln!(&mut std::io::stderr(), ">>> {} = {:?}", stringify!($var), $var);
    })
}

#[derive(Clone, Debug, PartialEq)]
enum Cell { Empty, Seal(usize), }

#[derive(Debug, PartialEq)]
enum CheckResult { Solved, NotYet, Invalid }

fn get_field(mut sc: &mut Scanner) -> Vec<Vec<Cell>> {
    let n: usize = sc.cin();
    assert!(n % 2 == 0);
    let mut fd = vec![vec![Cell::Empty; n]; n];
    for i in 0..n {
        let line: String = sc.cin();
        for (j, c) in line.chars().enumerate() {
            if c == '\n' { break }
            fd[i][j] = if c == '0' {
                Cell::Seal(0)
            } else if c == '1' {
                Cell::Seal(1)
            } else {
                Cell::Empty
            }
        }
    }
    fd
}

fn display(fd: &Vec<Vec<Cell>>) {
    let n = fd.len();
    for i in 0..n {
        for j in 0..n {
            print!("{}", if fd[i][j] == Cell::Seal(0) { '0' } else if fd[i][j] == Cell::Seal(1) { '1' } else { '.' });
        } println!("");
    }
}

fn check(fd: &Vec<Vec<Cell>>) -> CheckResult {
    let n = fd.len();

    // invalid checks

    { // each row and column contains same 0s and 1s
        for i in 0..n {
            let mut num: [usize; 2] = [0, 0];
            for j in 0..n {
                if let Cell::Seal(b) = fd[i][j] {
                    num[b] += 1;
                }
            }
            if num[0] > n / 2 { return CheckResult::Invalid }
            if num[1] > n / 2 { return CheckResult::Invalid }
        }
        for j in 0..n {
            let mut num: [usize; 2] = [0, 0];
            for i in 0..n {
                if let Cell::Seal(b) = fd[i][j] {
                    num[b] += 1;
                }
            }
            if num[0] > n / 2 { return CheckResult::Invalid }
            if num[1] > n / 2 { return CheckResult::Invalid }
        }
    }

    { // continuous 3 cells
        for i in 0..n {
            for j in 0..n {
                if fd[i][j] == Cell::Empty { continue }
                if j + 3 <= n {
                    if fd[i][j] == fd[i][j + 1] && fd[i][j] == fd[i][j + 2] { return CheckResult::Invalid }
                }
                if i + 3 <= n {
                    if fd[i][j] == fd[i + 1][j] && fd[i][j] == fd[i + 2][j] { return CheckResult::Invalid }
                }
            }
        }
    }

    { // should not contain same rows/columns
        {
            let mut memo = HashSet::new();
            for i in 0..n {
                let mut x = 1;
                let mut ok = true;
                for j in 0..n {
                    match fd[i][j] {
                        Cell::Empty => { ok = false; break },
                        Cell::Seal(b) => { x = 2 * x + b }
                    }
                }
                if ok {
                    if memo.contains(&x) {
                        return CheckResult::Invalid }
                    memo.insert(x);
                }
            }
        }
        {
            let mut memo = HashSet::new();
            for j in 0..n {
                let mut x = 1;
                let mut ok = true;
                for i in 0..n {
                    match fd[i][j] {
                        Cell::Empty => { ok = false; break },
                        Cell::Seal(b) => { x = 2 * x + b }
                    }
                }
                if ok {
                    if memo.contains(&x) {
                        return CheckResult::Invalid }
                    memo.insert(x);
                }
            }
        }
    }

    // valid

    for i in 0..n {
        for j in 0..n {
            if fd[i][j] == Cell::Empty { return CheckResult::NotYet }
        }
    }

    CheckResult::Solved
}

fn simple_solve(mut fd: &mut Vec<Vec<Cell>>) {

    const DI: [usize; 2] = [0, 1];
    const DJ: [usize; 2] = [1, 0];
    let n = fd.len();
    let mut changed = 0;

    {
        /*
         * neigh 2 cells
         * ".00." => "1001"
         */
        for i in 0..n {
            for j in 0..n {
                if let Cell::Seal(b) = fd[i][j] {
                    for k in 0..2 {
                        let i2 = i + DI[k];
                        let j2 = j + DJ[k];
                        if i2 >= n || j2 >= n { continue }
                        if fd[i][j] != fd[i2][j2] { continue }
                        {
                            let i3 = i + DI[k] * 2;
                            let j3 = j + DJ[k] * 2;
                            if i3 < n && j3 < n && fd[i3][j3] == Cell::Empty {
                                fd[i3][j3] = Cell::Seal(1 - b);
                                changed += 1;
                            }
                        }
                        {
                            if i >= DI[k] && j >= DJ[k] {
                                let i3 = i - DI[k];
                                let j3 = j - DJ[k];
                                if fd[i3][j3] == Cell::Empty {
                                    fd[i3][j3] = Cell::Seal(1 - b);
                                    changed += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    {
        /*
         * neigh 2 cells every 1
         * "1.1" => "101"
         */
        for i in 0..n {
            for j in 0..n {
                if let Cell::Seal(b) = fd[i][j] {
                    for k in 0..2 {
                        let i2 = i + DI[k] * 2;
                        let j2 = j + DJ[k] * 2;
                        if i2 >= n || j2 >= n { continue }
                        if fd[i2][j2] != fd[i][j] { continue }
                        let i3 = i + DI[k];
                        let j3 = j + DJ[k];
                        if fd[i3][j3] != Cell::Empty { continue }
                        fd[i3][j3] = Cell::Seal(1 - b);
                        changed += 1;
                    }
                }
            }
        }
    }

    if changed > 0 {
        simple_solve(&mut fd);
    }
}

fn choose(fd: &Vec<Vec<Cell>>) -> (usize, usize) {
    let n = fd.len();
    for i in 0..n {
        for j in 0..n {
            if fd[i][j] == Cell::Empty { return (i, j) }
        }
    }
    (0, 0) // dummy
}

fn solve(fd: Vec<Vec<Cell>>) {
    let mut stack = vec![fd];
    while let Some(fd) = stack.pop() {
        // display(&fd);
        match check(&fd) {
            CheckResult::Solved => {
                println!("SOLVED");
                display(&fd)
            },
            CheckResult::Invalid => continue,
            CheckResult::NotYet => {
                let (i, j) = choose(&fd);
                let mut fd0 = fd.clone(); fd0[i][j] = Cell::Seal(0); simple_solve(&mut fd0);
                let mut fd1 = fd.clone(); fd1[i][j] = Cell::Seal(1); simple_solve(&mut fd1);
                stack.push(fd0);
                stack.push(fd1);
            }
        }
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut fd = get_field(&mut sc);
    simple_solve(&mut fd);
    solve(fd);
}

#[allow(dead_code)]
struct Scanner { stdin: io::Stdin, buffer: VecDeque<String>, }
#[allow(dead_code)]
impl Scanner {
    fn new() -> Scanner { Scanner { stdin: io::stdin(), buffer: VecDeque::new() } }
    fn reserve(&mut self) {
        while self.buffer.len() == 0 {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        self.reserve();
        match self.buffer.pop_front().unwrap().parse::<T>() {
            Ok(a) => a,
            Err(_) => panic!("parse err")
        }
    }
    fn get_char(&mut self) -> char {
        self.reserve();
        let head = self.buffer[0].chars().nth(0).unwrap();
        let tail = String::from( &self.buffer[0][1..] );
        if tail.len()>0 { self.buffer[0]=tail } else { self.buffer.pop_front(); }
        head
    }
}
