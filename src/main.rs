use std::{time::Duration, thread::sleep};

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn expensive_calculation<T>(_n: T) {
    sleep(Duration::from_secs(1));
}

struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char)
}

struct Progress<T, B> {
    iter: T,
    i: usize,
    bound: B
}

trait ProgressDisplay: Sized {
    fn display<T>(&self, progress: &Progress<T, Self>);
}

impl ProgressDisplay for Bounded {
    fn display<T>(&self, progress: &Progress<T, Self>) {
	println!("{}{}{}{}",
		 self.delims.0,
		 "*".repeat(progress.i ),
		 " ".repeat(self.bound - progress.i),
		 self.delims.1);
    }    
}

impl ProgressDisplay for Unbounded {
    fn display<T>(&self, progress: &Progress<T, Self>) {
	println!("{}","*".repeat(progress.i ));
    }    
}

impl<T> Progress<T, Unbounded> {
    fn new(iter: T) -> Self {
        Progress { iter, i: 0, bound: Unbounded }
    }   
}

impl<T> Progress<T, Unbounded>
where T: ExactSizeIterator {
    pub fn with_bound(mut self) -> Progress<T, Bounded> {
	let bound = Bounded {
	    bound: self.iter.len(),
	    delims: ('[',']')
	};
	Progress { i: self.i, iter: self.iter, bound}
    }
}

impl<T,B> Iterator for Progress<T,B> 
where T:Iterator, B: ProgressDisplay {
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        clear_screen();
        self.bound.display(&self);
        
	self.i += 1;
        self.iter.next()
    }
}

impl<T> Progress<T, Bounded> {
    pub fn with_delims(mut self, delims: (char, char))  -> Self {
	self.bound.delims = delims;
	self
    }
}

trait ProgressIterExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<T> ProgressIterExt for T {
    fn progress(self) -> Progress<Self, Unbounded>{
        Progress::new(self)
    }
}

fn main() {
    let brkts = ('{','}');
    let v = vec![1,2,3];
    // progress(v.iter(), expensive_calculation);
    for n in v.iter()
	.progress().with_bound().with_delims(brkts) {
        expensive_calculation(n);
    }

    sleep(Duration::from_secs(1));

    use std::collections::HashSet;
    let mut h = HashSet::new();
    h.insert(0);
    // progress(h.iter(), expensive_calculation);
    for n in h.iter().progress().with_bound() {
        expensive_calculation(n);
    }

    sleep(Duration::from_secs(1));

    let arr = ["hi","there"];
    match  arr.first() {
        Some(word) =>  
            {
                for x in word.chars()
		    // .collect::<Vec<char>>().iter()
		    .progress() {
                            expensive_calculation(x);
                            print!("{}!", x);
                        }
            }
        None => print!("arr is empty")
    }
    
}
