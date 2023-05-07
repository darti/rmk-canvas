use derive_builder::Builder;

#[derive(Builder, Debug, PartialEq)]
pub struct Notebook {
    version: usize,
    nb_layers: u32,
    #[builder(default, setter(each(name = "layer")))]
    layers: Vec<Layer>,
}

#[derive(Builder, Debug, PartialEq, Clone)]
pub struct Layer {
    #[builder(default)]
    nb_strokes: u32,

    #[builder(default, setter(each(name = "stroke")))]
    strokes: Vec<Stroke>,
}

//Now a line has the following attributes: 4
//  Bytes int32_t for the brush type,
// 4 Bytes int32_t for the color of the line,
// and an other 4 Bytes “padding” which in all my files is zero.
// Then follows the base brush size in 4 Bytes float32_t
// aaaannd the number of points inside the current line as int32_t (4 Bytes).

#[derive(Builder, Debug, PartialEq, Clone)]
pub struct Stroke {
    #[builder(default)]
    brush: u32,

    #[builder(default)]
    color: u32,

    #[builder(default)]
    base_brush_size: f32,

    #[builder(default)]
    nb_points: u32,

    #[builder(default, setter(each(name = "point")))]
    points: Vec<Point>,
}

#[derive(Builder, Debug, PartialEq, Clone)]
pub struct Point {
    #[builder(default)]
    x: f32,

    #[builder(default)]
    y: f32,

    #[builder(default)]
    speed: f32,

    #[builder(default)]
    tilt: f32,

    #[builder(default)]
    width: f32,

    #[builder(default)]
    pressure: f32,
}
