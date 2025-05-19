//notifs.rs 

///Tells you that something just happened.
#[derive(Debug)]
pub enum Notif {
    Log(String),
    Warning(String),
    Disaster(String), //well, that just happened *laugh track*
}                     //i fucking hate forced irony in movies
                      //yes this is a side tangent and you
                      //will suffer for it

///wraps the value inside of a container telling you
///what happened along the way.
///Intended to be used as Result<Success<T>, Disaster>
#[derive(Debug)]
pub struct Success<T> {
    pub val: T,
    pub notifs: Vec<Notif>,
}
