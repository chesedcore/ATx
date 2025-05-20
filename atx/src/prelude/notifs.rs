//notifs.rs 

///Tells you that something just happened.
#[derive(Debug)]
pub enum Notif {
    Log(String),
    Warning(String),
}

pub fn log(string: &str) -> Notif {
    Notif::Log(string.to_string())
}
pub fn warning(string: &str) -> Notif {
    Notif::Warning(string.to_string())
}

pub struct Disaster(pub String); 

///wraps the value inside of a container telling you
///what happened along the way.
///Intended to be used as Result<Success<T>, Disaster>
#[derive(Debug)]
pub struct Success<T> {
    pub val: T,
    pub notifs: Vec<Notif>,
}

impl<T> Success<T> {
    ///default constructor: wraps T.
    pub fn new(val: T) -> Self {
        Success{val, notifs: vec![]}
    }

    ///consumes this struct and chains the notif vec into a new type N
    pub fn chain<N>(self, new_value: N) -> Success<N> {
        Success{ val: new_value, notifs: self.notifs }
    }

    ///attach the log into the vector...
    pub fn log(mut self, message: &str) -> Self {
        self.notifs.push(log(message));
        self
    }

    ///attach the warning into the vector...
    pub fn warn(mut self, message: &str) -> Self {
        self.notifs.push(warning(message));
        self
    }
}
