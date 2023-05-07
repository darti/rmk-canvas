use nom::IResult;

use crate::{
    errors::RmkFormatError,
    format::{Layer, LayerBuilder, Notebook},
    format::{NotebookBuilder, Stroke},
};

use super::{common::parse_u32, ParseResult};

pub fn parse(i: &[u8]) -> Result<Notebook, RmkFormatError> {
    let (i, nb_layers) = nb_layers(i)?;

    let mut nb = NotebookBuilder::default();
    nb.version(5).nb_layers(nb_layers);

    let mut i = i;

    for _l in 0..nb_layers {
        let (il, layer) = parse_layer(i)?;
        i = il;
        nb.layer(layer);
    }

    nb.build().map_err(|e| e.into())
}

fn nb_layers(i: &[u8]) -> ParseResult<u32> {
    parse_u32(i)
}

fn nb_strokes(i: &[u8]) -> ParseResult<u32> {
    parse_u32(i)
}

fn brush(i: &[u8]) -> ParseResult<u32> {
    parse_u32(i)
}

fn parse_layer(i: &[u8]) -> ParseResult<Layer> {
    let (i, nb_strokes) = nb_strokes(i).unwrap();

    let mut layer = LayerBuilder::default();
    layer.nb_strokes(nb_strokes);

    let mut i = i;

    for _l in 0..nb_strokes {
        let (is, stroke) = parse_stroke(i).unwrap();
        i = is;

        layer.stroke(stroke);
    }

    Ok((i, layer.build()?))
}

fn parse_stroke(i: &[u8]) -> IResult<&[u8], Stroke> {
    todo!()
}
