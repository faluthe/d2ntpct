use std::{time::Instant, env, collections::HashMap, thread};

#[derive(Clone)]
struct Equation {
    minuend: String,
    subtrahend: String,
    difference: String,
}

fn factorial(x: usize) -> usize {
    if x == 1 {
        x
    } else {
        x * factorial(x - 1)
    }
}

fn permute(mut k: usize, mut string: Vec<char>) -> String {
    for i in 1..string.len() {
        string.swap(k % (i + 1), i);
        k = k / (i + 1);
    }
    string.into_iter().collect()
}

fn convert(letters: &String, key: &String) -> usize {
    let mut x = 0;
    for l in 0..letters.len() {
        let theletter = letters.chars().nth(l).unwrap();
        // println!("theletter: {theletter}");
        x += key.find(theletter).unwrap() * key.len().pow((letters.len() - 1 - l).try_into().unwrap());
    }
    x
}

fn attempt(equations: &Vec<Equation>, permutation: &String) -> bool {
    for e in equations {
        let minuend = convert(&e.minuend, permutation);
        let subtrahend = convert(&e.subtrahend, permutation);
        let difference = convert(&e.difference, permutation);

        if subtrahend > minuend || difference > minuend {
            return false;
        }
        if minuend - subtrahend == difference {
            return true;
        }
    }
    false
}

fn verify(equations: &Vec<Equation>, permutation: &String) -> bool {
    for e in equations {
        let minuend = convert(&e.minuend, permutation);
        let subtrahend = convert(&e.subtrahend, permutation);
        let difference = convert(&e.difference, permutation);

        if subtrahend > minuend || difference > minuend {
            return false;
        }
        if minuend - subtrahend != difference {
            return false;
        }
    }
    true
}

fn thread_begin(start: usize, end: usize, key: String, equations: Vec<Equation>, timer: &Instant) {
    for i in start..end {
        let permutation = permute(i, key.chars().collect());
        if attempt(&equations, &permutation) && verify(&equations, &permutation) {
            println!("Verified: {} in {}ms", permutation, timer.elapsed().as_millis());
        }
    }
}

fn main() {
    // Start timing
    let timer = Instant::now();

    // Parse options
    let args: Vec<String> = env::args().collect();
    let useage = format!("Useage: {} [threads] [known letters]", args[0]);
    let mut threads = 1;
    // Iterate by two after and error if cant parse for now
    let mut known_letters = HashMap::new();
    let mut args = args.iter();
    args.next();
    match args.next() {
        Some(a) => threads = usize::from_str_radix(&a, 10).expect(&useage),
        None => (),
    }
    while let Some(a) = args.next() {
        let cur = a;
        match args.next() {
            Some(next) => {
                // TEST THIS
                let letter: char = cur.parse().expect("Not a char");
                let num: usize = next.parse().expect("Not an i32");
                known_letters.insert(letter, num);
            },
            None => panic!("{}", &useage),
        }
    }
    println!("Threads: {threads}");
    for letter in known_letters {
        println!("{} : {}", letter.0, letter.1);
    }


    // Open puzzle file
    // Parse puzzle file into equations
    // Placeholder
    let key = String::from("ACFQUTWEXH");
    // remove here......
    let equations = vec![
        Equation{ minuend: String::from("CUTAX"), subtrahend: String::from("HQWQU"), difference: String::from("UFAC")},
        Equation{ minuend: String::from("UFACH"), subtrahend: String::from("WQHQC"), difference: String::from("FFCQ")},
        Equation{ minuend: String::from("FFCQU"), subtrahend: String::from("UQAQH"), difference: String::from("QHXT")},
        Equation{ minuend: String::from("QHXTX"), subtrahend: String::from("FQXQA"), difference: String::from("HQTQ")},
    ];
    // let base = key.len();
    // Divide into threads
    let mut children = Vec::new();
    let scale = factorial(key.len()) / threads;
    for t in 0..threads {
        let start = t * scale;
        let end = (t + 1) * scale;
        println!("Thread {}: {} - {}", t + 1, start, end);
        let key = key.clone();
        // restore here..........
        let equations = equations.clone();
        let thread = thread::spawn(move || { thread_begin(start, end, key, equations, &timer) });
        children.push(thread);
    }

    // Join threads
    for t in children {
        t.join().unwrap();
    }

    // Finish timing
    println!("{}ms", timer.elapsed().as_millis());
}