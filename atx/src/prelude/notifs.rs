//notifs.rs 

///Tells you that something just happened.
#[derive(Debug)]
pub enum Notif {
    Log(String),
    Warning(String),
}

pub fn Log(string: &str) -> Notif {
    Notif::Log(string.to_string())
}
pub fn Warning(string: &str) -> Notif {
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
    pub fn log(mut self, log: &str) -> Self {
        self.notifs.push(Log(log));
        self
    }

    ///attach the warning into the vector...
    pub fn warn(mut self, warning: &str) -> Self {
        self.notifs.push(Warning(warning));
        self
    }
}
