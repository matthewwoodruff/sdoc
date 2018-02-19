#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub struct Request<'a> {
    pub current: Option<&'a String>,
    pub args: &'a [String],
    all_args: &'a Vec<String>,
    shift: usize,
    completed: Option<usize>,
}

impl <'a> Request<'a> {
    pub fn new(args: &Vec<String>, completed: Option<usize>) -> Request {
        Request::build(0, completed, args)
    }

    pub fn next(self) -> Request<'a> {
        Request::build(self.shift + 1, self.completed, self.all_args)
    }

    fn build(shift: usize, completed: Option<usize>, all_args: &Vec<String>) -> Request {
        let u = all_args.len();
        if u - shift <= 0 {
            Request {
                shift,
                all_args,
                current: None ,
                args: &all_args[0..0],
                completed
            }
        } else {
            Request {
                shift,
                all_args,
                current: Some(&all_args[shift]) ,
                args: &all_args[shift+1..u],
                completed
            }
        }
    }

    pub fn autocomplete(&self) -> bool {
        self.completed.map( |v| v == self.shift).unwrap_or(false)
    }
    pub fn autocomplete_enabled(&self) -> bool {
        self.completed.map( |v| v > 0).unwrap_or(false)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Response {
    Ok,
    Err(i32)
}