mod channel;
mod env;
mod error;
mod eval;
mod operators;
mod parse;

pub use channel::Channel;
pub use env::Environment;
pub use error::Error;
pub use eval::{args, args_min, eval};
pub use parse::parse;
use screech::Signal;

#[derive(Debug, Clone)]
pub enum Blad {
    Atom(String),
    Keyword(Keyword),
    Lambda(Environment, Vec<String>, Box<Blad>),
    List(Vec<Blad>),
    Literal(Literal),
    Macro(Vec<String>, Box<Blad>),
    Quote(Box<Blad>),
    Screech(Screech),
    Symbol(String),
    Unit,
}

impl Blad {
    pub fn get_atom(&self) -> Result<&str, Error> {
        match self {
            Blad::Atom(s) => Ok(s),
            _ => Err(Error::ExpectedAtom(self.clone())),
        }
    }

    pub fn get_list(&self) -> Result<&[Blad], Error> {
        match self {
            Blad::List(list) => Ok(list),
            _ => Err(Error::ExpectedList(self.clone())),
        }
    }

    pub fn get_symbol(&self) -> Result<&str, Error> {
        match self {
            Blad::Symbol(symbol) => Ok(symbol),
            _ => Err(Error::ExpectedSymbol(self.clone())),
        }
    }

    pub fn get_module(&self) -> Result<usize, Error> {
        match self {
            Blad::Screech(Screech::Module(id)) => Ok(*id),
            _ => Err(Error::ExpectedScreechModule(self.clone())),
        }
    }

    pub fn get_signal(&self) -> Result<Signal, Error> {
        match self {
            Blad::Screech(Screech::Signal(signal)) => Ok(*signal),
            _ => Err(Error::ExpectedScreechSignal(self.clone())),
        }
    }

    pub fn get_usize(&self) -> Result<usize, Error> {
        match self {
            Blad::Literal(Literal::Usize(int)) => Ok(*int),
            _ => Err(Error::ExpectedUsize(self.clone())),
        }
    }

    pub fn get_f32(&self) -> Result<f32, Error> {
        match self {
            Blad::Literal(Literal::F32(f)) => Ok(*f),
            _ => Err(Error::ExpectedF32(self.clone())),
        }
    }

    pub fn to_pitch(&self) -> Result<f32, Error> {
        match self {
            Blad::Atom(s) => atom_to_pitch(s),
            _ => Err(Error::ExpectedAtom(self.clone())),
        }
    }
}

impl PartialEq for Blad {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Blad::Atom(a), Blad::Atom(b)) => a == b,
            (Blad::Keyword(a), Blad::Keyword(b)) => a == b,
            (Blad::List(a), Blad::List(b)) => a == b,
            (Blad::Literal(a), Blad::Literal(b)) => a == b,
            (Blad::Quote(a), Blad::Quote(b)) => a == b,
            (Blad::Screech(a), Blad::Screech(b)) => a == b,
            (Blad::Symbol(a), Blad::Symbol(b)) => a == b,
            (Blad::Unit, Blad::Unit) => true,
            _ => false,
        }
    }
}

fn atom_to_pitch(atom: &str) -> Result<f32, Error> {
    match atom {
        ":a0" => Ok(27.5000),
        ":a#0" => Ok(29.1352),
        ":b0" => Ok(30.8677),
        ":c1" => Ok(32.7032),
        ":c#1" => Ok(34.6478),
        ":d1" => Ok(36.7081),
        ":d#1" => Ok(38.8909),
        ":e1" => Ok(41.2034),
        ":f1" => Ok(43.6535),
        ":f#1" => Ok(46.2493),
        ":g1" => Ok(48.9994),
        ":g#1" => Ok(51.9131),
        ":a1" => Ok(55.0000),
        ":a#1" => Ok(58.2705),
        ":b1" => Ok(61.7354),
        ":c2" => Ok(65.4064),
        ":c#2" => Ok(69.2957),
        ":d2" => Ok(73.4162),
        ":d#2" => Ok(77.7817),
        ":e2" => Ok(82.4069),
        ":f2" => Ok(87.3071),
        ":f#2" => Ok(92.4986),
        ":g2" => Ok(97.9989),
        ":g#2" => Ok(103.8262),
        ":a2" => Ok(110.0000),
        ":a#2" => Ok(116.5409),
        ":b2" => Ok(123.4708),
        ":c3" => Ok(130.8128),
        ":c#3" => Ok(138.5913),
        ":d3" => Ok(146.8324),
        ":d#3" => Ok(155.5635),
        ":e3" => Ok(164.8138),
        ":f3" => Ok(174.6141),
        ":f#3" => Ok(184.9972),
        ":g3" => Ok(195.9977),
        ":g#3" => Ok(207.6523),
        ":a3" => Ok(220.0000),
        ":a#3" => Ok(233.0819),
        ":b3" => Ok(246.9417),
        ":c4" => Ok(261.6256),
        ":c#4" => Ok(277.1826),
        ":d4" => Ok(293.6648),
        ":d#4" => Ok(311.1270),
        ":e4" => Ok(329.6276),
        ":f4" => Ok(349.2282),
        ":f#4" => Ok(369.9944),
        ":g4" => Ok(391.9954),
        ":g#4" => Ok(415.3047),
        ":a4" => Ok(440.0000),
        ":a#4" => Ok(466.1638),
        ":b4" => Ok(493.8833),
        ":c5" => Ok(523.2511),
        ":c#5" => Ok(554.3653),
        ":d5" => Ok(587.3295),
        ":d#5" => Ok(622.2540),
        ":e5" => Ok(659.2551),
        ":f5" => Ok(698.4565),
        ":f#5" => Ok(739.9888),
        ":g5" => Ok(783.9909),
        ":g#5" => Ok(830.6094),
        ":a5" => Ok(880.0000),
        ":a#5" => Ok(932.3275),
        ":b5" => Ok(987.7666),
        ":c6" => Ok(1046.5023),
        ":c#6" => Ok(1108.7305),
        ":d6" => Ok(1174.6591),
        ":d#6" => Ok(1244.5079),
        ":e6" => Ok(1318.5102),
        ":f6" => Ok(1396.9129),
        ":f#6" => Ok(1479.9777),
        ":g6" => Ok(1567.9817),
        ":g#6" => Ok(1661.2188),
        ":a6" => Ok(1760.0000),
        ":a#6" => Ok(1864.6550),
        ":b6" => Ok(1975.5332),
        ":c7" => Ok(2093.0045),
        ":c#7" => Ok(2217.4610),
        ":d7" => Ok(2349.3181),
        ":d#7" => Ok(2489.0159),
        ":e7" => Ok(2637.0205),
        ":f7" => Ok(2793.8259),
        ":f#7" => Ok(2959.9554),
        ":g7" => Ok(3135.9635),
        ":g#7" => Ok(3322.4376),
        ":a7" => Ok(3520.0000),
        ":a#7" => Ok(3729.3101),
        ":b7" => Ok(3951.0664),
        ":c8" => Ok(4186.0090),
        _ => Err(Error::InvalidNote(atom.into())),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Usize(usize),
    F32(f32),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Add,
    Append,
    Cons,
    Do,
    Equal,
    GreaterThan,
    Head,
    If,
    Lambda,
    LessThan,
    Let,
    List,
    Macro,
    Subtract,
    Tail,
    Cast,
    Call,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screech {
    Module(usize),
    Signal(Signal),
}
