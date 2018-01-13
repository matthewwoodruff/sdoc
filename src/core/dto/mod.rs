#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub struct Request<'a> {
    args: &'a Vec<String>,
    count: usize,
    completed: Option<usize>,
    pub current: Option<&'a String>,
    pub next: &'a [String]
}

impl <'a> Request<'a> {
    pub fn new(args: &Vec<String>, completed: Option<usize>) -> Request {
        Request::build(0, completed, args)
    }

    pub fn next(self) -> Request<'a> {
        Request::build(self.count + 1, self.completed, self.args)
    }

    fn build(count: usize, completed: Option<usize>, args: &Vec<String>) -> Request {
        let u = args.len();
        if u - count <= 0 {
            Request {
                count,
                args,
                current: None ,
                next: &args[0..0],
                completed
            }
        } else {
            Request {
                count,
                args,
                current: Some(&args[count]) ,
                next: &args[count+1..u],
                completed
            }
        }
    }

    pub fn autocomplete(&self) -> bool {
        self.completed.map( |v| v == self.count).unwrap_or(false)
    }
    pub fn autocomplete_enabled(&self) -> bool {
        self.completed.map( |v| v > 0).unwrap_or(false)
    }
}

#[derive(Debug, PartialEq)]
pub enum Response {
    Ok,
    Err(i32)
}