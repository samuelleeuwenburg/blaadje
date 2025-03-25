use super::super::{args, eval};
use crate::{Blad, Environment, Error, Literal};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use wavv::{Data, Wav};

pub fn process_samples(list: &[Blad], env: Arc<Mutex<Environment>>) -> Result<Blad, Error> {
    args(list, 1)?;

    let result = eval(&list[0], env.clone())?;
    let path = result.get_string()?;

    let bytes = fs::read(Path::new(path)).map_err(|_| Error::FileError)?;
    let wav = Wav::from_bytes(&bytes).map_err(|_| Error::WavError)?;

    match wav.data {
        Data::BitDepth8(samples) => Ok(normalize_u8(&samples)),
        Data::BitDepth16(samples) => Ok(normalize_i16(&samples)),
        Data::BitDepth24(samples) => Ok(normalize_i32(&samples)),
    }
}

fn normalize_u8(samples: &[u8]) -> Blad {
    let mut normalized = vec![];

    for s in samples {
        let sample = (*s as f32 / u8::MAX as f32) - 1.0;
        normalized.push(Blad::Literal(Literal::F32(sample)));
    }

    Blad::List(normalized)
}

fn normalize_i16(samples: &[i16]) -> Blad {
    let mut normalized = vec![];

    for s in samples {
        let sample = *s as f32 / i16::MAX as f32;
        normalized.push(Blad::Literal(Literal::F32(sample)));
    }

    Blad::List(normalized)
}

fn normalize_i32(samples: &[i32]) -> Blad {
    let mut normalized = vec![];

    for s in samples {
        let sample = *s as f32 / 8_388_607.0 as f32;
        normalized.push(Blad::Literal(Literal::F32(sample)));
    }

    Blad::List(normalized)
}
