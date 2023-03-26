use std::{time::Duration, thread::sleep};

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn expensive_calculation<T>(_n: T) {
    sleep(Duration::from_secs(1));
}

struct Progress<T> {
    iter: T,
    i: usize,
    bound: Option<usize>
}
impl<T> Progress<T> {
    fn new(iter: T) -> Self {
        Progress { iter, i: 0, bound: None }
    }  
}

impl<T> Progress<T>
where T: ExactSizeIterator {
    pub fn with_bound(mut self) -> Self {
	self.bound = Some (self.iter.len());
	self
    }
}

impl<T> Iterator for Progress<T> 
where T:Iterator {
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        clear_screen();
       
        match self.bound {
	    Some(bound) =>
		println!("[{}{}]","*".repeat(self.i )," ".repeat(bound - self.i)),
	    None =>  println!("{}","*".repeat(self.i ))
	}
	self.i += 1;
        self.iter.next()
    }
}

trait ProgressIterExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<T> ProgressIterExt for T {
    fn progress(self) -> Progress<Self>{
        Progress::new(self)
    }
}

fn main() {
    let v = vec![1,2,3];
    // progress(v.iter(), expensive_calculation);
    for n in v.iter().progress().with_bound() {
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
		    .collect::<Vec<char>>().iter()
		    .progress().with_bound() {
                            expensive_calculation(x);
                            print!("{}!", x);
                        }
            }
        None => print!("arr is empty")
    }
    
}
