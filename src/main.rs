use std::fs;

const SQRT_3: f64 = 1.732050807568877293527446341505872367_f64;
const TRIANGLE_WIDTH: f64 = 300_f64;
const TRIANGLE_HEIGHT: f64 = TRIANGLE_WIDTH * SQRT_3 / 2_f64;
const IMAGE_WIDTH_IN_TRIANGLES: f64 = 2_f64;
const IMAGE_HEIGHT_IN_TRIANGLES: f64 = 2_f64;
const IMAGE_WIDTH: f64 = TRIANGLE_WIDTH * IMAGE_WIDTH_IN_TRIANGLES;
const IMAGE_HEIGHT: f64 = TRIANGLE_HEIGHT * IMAGE_HEIGHT_IN_TRIANGLES;

fn main() -> std::io::Result<()> {
    let stroke_width = 15_f64;
    let narrowed_stroke_width = stroke_width * 0.9;
    let inner_triangle_start_x = SQRT_3 * stroke_width / 2_f64;
    let inner_triangle_start_y = TRIANGLE_HEIGHT - stroke_width / 2_f64;
    let inner_triangle_width = TRIANGLE_WIDTH - SQRT_3 * stroke_width;
    let inner_triangle_half_width = inner_triangle_width / 2_f64;
    let inner_triangle_height = inner_triangle_width * SQRT_3 / 2_f64;
    let triangle_half_width = TRIANGLE_WIDTH / 2_f64;

    fs::write("tetrahedron-net.svg", format!(r##"<svg width="{IMAGE_WIDTH}" height="{IMAGE_HEIGHT}" viewBox="0 0 {IMAGE_WIDTH} {IMAGE_HEIGHT}" xmlns="http://www.w3.org/2000/svg" version="2">
    <defs>
        <g id="triangle-tile">
            <path d="M {inner_triangle_start_x},{inner_triangle_start_y} h {inner_triangle_width} l -{inner_triangle_half_width},-{inner_triangle_height} z" fill="none" stroke="grey" stroke-width="{narrowed_stroke_width}" />
        </g>
    </defs>
    <use href="#triangle-tile" x="{triangle_half_width}" y="0" />
    <use href="#triangle-tile" x="{triangle_half_width}" y="0" transform="rotate(180,{TRIANGLE_WIDTH},{TRIANGLE_HEIGHT})" />
    <use href="#triangle-tile" x="0" y="{TRIANGLE_HEIGHT}" />
    <use href="#triangle-tile" x="{TRIANGLE_WIDTH}" y="{TRIANGLE_HEIGHT}" />
</svg>
"##))?;

    let triangle_quarter_width = TRIANGLE_WIDTH / 4_f64;
    let triangle_half_height = TRIANGLE_HEIGHT / 2f64;
    let wide_arc_width = 40;
    let narrow_arc_width = 20;
    fs::write("tiles.svg", format!(r##"<svg width="{IMAGE_WIDTH}" height="{IMAGE_HEIGHT}" viewBox="0 0 {IMAGE_WIDTH} {IMAGE_HEIGHT}" xmlns="http://www.w3.org/2000/svg" version="2">
    <defs>
        <path id="arc" d="M {triangle_quarter_width},{triangle_half_height} a {triangle_half_width} {triangle_half_width} 60 0 0 {triangle_half_width},0" fill="none" />
        <g id="triangle-tile-with-arc">
            <path d="M {inner_triangle_start_x},{inner_triangle_start_y} h {inner_triangle_width} l -{inner_triangle_half_width},-{inner_triangle_height} z" fill="none" stroke="grey" stroke-width="{stroke_width}" />
            <use href="#arc" stroke="#e4a" stroke-width="{wide_arc_width}" />
            <use href="#arc" stroke="#a7e" stroke-width="{narrow_arc_width}" />
        </g>
    </defs>
    <use href="#triangle-tile-with-arc" x="0" y="0" />
    <use href="#triangle-tile-with-arc" x="{TRIANGLE_WIDTH}" y="0" />
    <use href="#triangle-tile-with-arc" x="0" y="{TRIANGLE_HEIGHT}" />
    <use href="#triangle-tile-with-arc" x="{TRIANGLE_WIDTH}" y="{TRIANGLE_HEIGHT}" />
</svg>
"##))?;

    Ok(())
}
