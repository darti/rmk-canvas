use nom::{
    bytes::complete::{tag, take, take_while1},
    character::is_digit,
    combinator::map,
    IResult,
};

use std::str::from_utf8;

use crate::{
    errors::RmkFormatError,
    format::{Notebook, NotebookBuilder},
};

pub fn notebook(i: &[u8]) -> Result<Notebook, RmkFormatError> {
    let (i, version) = version(i).map_err(|e| RmkFormatError::ParserError(e.to_owned()))?;

    let nb = NotebookBuilder::default().version(version).build()?;

    Ok(nb)
}

fn parse_int<T>(i: &[u8]) -> IResult<&[u8], T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    map(take_while1(is_digit), |b| {
        from_utf8(b).unwrap().parse::<T>().unwrap()
    })(i)
}

fn version(i: &[u8]) -> IResult<&[u8], usize> {
    let (i, r) = take(43usize)(i)?;
    let (r, _) = tag("reMarkable .lines file, version=")(r)?;
    let (_, version) = parse_int(r)?;

    Ok((i, version))
}
