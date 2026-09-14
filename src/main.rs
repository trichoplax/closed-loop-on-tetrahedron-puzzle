use std::fs;

const SQRT_3: f64 = 1.732050807568877293527446341505872367_f64;

fn main() -> std::io::Result<()> {
    let tetrahedron_triangle_width = 300_f64;
    let tetrahedron_triangle_height = tetrahedron_triangle_width * SQRT_3 / 2_f64;
    let tetrahedron_net_image_width_in_triangles = 2_f64;
    let tetrahedron_net_image_height_in_triangles = 2_f64;
    let tetrahedron_net_image_width = tetrahedron_triangle_width * tetrahedron_net_image_width_in_triangles;
    let tetrahedron_net_image_height = tetrahedron_triangle_height * tetrahedron_net_image_height_in_triangles;
    let tetrahedron_triangle_stroke_width = 15_f64;
    let tetrahedron_triangle_narrowed_stroke_width = tetrahedron_triangle_stroke_width * 0.9;
    let tetrahedron_inner_triangle_start_x = SQRT_3 * tetrahedron_triangle_stroke_width / 2_f64;
    let tetrahedron_inner_triangle_start_y = tetrahedron_triangle_height - tetrahedron_triangle_stroke_width / 2_f64;
    let tetrahedron_inner_triangle_width = tetrahedron_triangle_width - SQRT_3 * tetrahedron_triangle_stroke_width;
    let tetrahedron_inner_triangle_half_width = tetrahedron_inner_triangle_width / 2_f64;
    let tetrahedron_inner_triangle_height = tetrahedron_inner_triangle_width * SQRT_3 / 2_f64;
    let tetrahedron_triangle_half_width = tetrahedron_triangle_width / 2_f64;

    fs::write("tetrahedron-net.svg", format!(r##"<svg width="{tetrahedron_net_image_width}" height="{tetrahedron_net_image_height}" viewBox="0 0 {tetrahedron_net_image_width} {tetrahedron_net_image_height}" xmlns="http://www.w3.org/2000/svg" version="2">
    <defs>
        <g id="triangle-tile">
            <path d="M {tetrahedron_inner_triangle_start_x},{tetrahedron_inner_triangle_start_y} h {tetrahedron_inner_triangle_width} l -{tetrahedron_inner_triangle_half_width},-{tetrahedron_inner_triangle_height} z" fill="none" stroke="grey" stroke-width="{tetrahedron_triangle_narrowed_stroke_width}" />
        </g>
    </defs>
    <use href="#triangle-tile" x="{tetrahedron_triangle_half_width}" y="0" />
    <use href="#triangle-tile" x="{tetrahedron_triangle_half_width}" y="0" transform="rotate(180,{tetrahedron_triangle_width},{tetrahedron_triangle_height})" />
    <use href="#triangle-tile" x="0" y="{tetrahedron_triangle_height}" />
    <use href="#triangle-tile" x="{tetrahedron_triangle_width}" y="{tetrahedron_triangle_height}" />
</svg>
"##))?;

    let tetrahedron_tiles_image_width_in_triangles = 2f64;
    let tetrahedron_tiles_image_height_in_triangles = 2f64;
    let tetrahedron_tiles_image_width = tetrahedron_triangle_width * tetrahedron_tiles_image_width_in_triangles;
    let tetrahedron_tiles_image_height = tetrahedron_triangle_height * tetrahedron_tiles_image_height_in_triangles;
    let tetrahedron_triangle_quarter_width = tetrahedron_triangle_width / 4_f64;
    let tetrahedron_triangle_half_height = tetrahedron_triangle_height / 2f64;
    let tetrahedron_wide_arc_width = 40;
    let tetrahedron_narrow_arc_width = 20;
    fs::write("tetrahedron-tiles.svg", format!(r##"<svg width="{tetrahedron_tiles_image_width}" height="{tetrahedron_tiles_image_height}" viewBox="0 0 {tetrahedron_tiles_image_width} {tetrahedron_tiles_image_height}" xmlns="http://www.w3.org/2000/svg" version="2">
    <defs>
        <path id="arc" d="M {tetrahedron_triangle_quarter_width},{tetrahedron_triangle_half_height} a {tetrahedron_triangle_half_width} {tetrahedron_triangle_half_width} 60 0 0 {tetrahedron_triangle_half_width},0" fill="none" />
        <g id="triangle-tile-with-arc">
            <path d="M {tetrahedron_inner_triangle_start_x},{tetrahedron_inner_triangle_start_y} h {tetrahedron_inner_triangle_width} l -{tetrahedron_inner_triangle_half_width},-{tetrahedron_inner_triangle_height} z" fill="none" stroke="grey" stroke-width="{tetrahedron_triangle_stroke_width}" />
            <use href="#arc" stroke="#e4a" stroke-width="{tetrahedron_wide_arc_width}" />
            <use href="#arc" stroke="#a7e" stroke-width="{tetrahedron_narrow_arc_width}" />
        </g>
    </defs>
    <use href="#triangle-tile-with-arc" x="0" y="0" />
    <use href="#triangle-tile-with-arc" x="{tetrahedron_triangle_width}" y="0" />
    <use href="#triangle-tile-with-arc" x="0" y="{tetrahedron_triangle_height}" />
    <use href="#triangle-tile-with-arc" x="{tetrahedron_triangle_width}" y="{tetrahedron_triangle_height}" />
</svg>
"##))?;

    let octahedron_triangle_width = 150_f64;
    let octahedron_triangle_height = octahedron_triangle_width * SQRT_3 / 2_f64;
    let octahedron_net_image_width_in_triangles = 3.5_f64;
    let octahedron_net_image_height_in_triangles = 3_f64;
    let octahedron_net_image_width = octahedron_triangle_width * octahedron_net_image_width_in_triangles;
    let octahedron_net_image_height = octahedron_triangle_height * octahedron_net_image_height_in_triangles;
    let octahedron_triangle_stroke_width = 15_f64;
    let octahedron_triangle_narrowed_stroke_width = octahedron_triangle_stroke_width * 0.9;
    let octahedron_inner_triangle_start_x = SQRT_3 * octahedron_triangle_stroke_width / 2_f64;
    let octahedron_inner_triangle_start_y = octahedron_triangle_height - octahedron_triangle_stroke_width / 2_f64;
    let octahedron_inner_triangle_width = octahedron_triangle_width - SQRT_3 * octahedron_triangle_stroke_width;
    let octahedron_inner_triangle_half_width = octahedron_inner_triangle_width / 2_f64;
    let octahedron_inner_triangle_height = octahedron_inner_triangle_width * SQRT_3 / 2_f64;
    let octahedron_triangle_half_width = octahedron_triangle_width / 2_f64;
    let octahedron_triangle_three_halves_width = octahedron_triangle_width * 3_f64 / 2_f64;
    let octahedron_triangle_five_halves_width = octahedron_triangle_width * 5_f64 / 2_f64;
    let octahedron_triangle_half_height = octahedron_triangle_height / 2f64;
    let octahedron_triangle_double_width = octahedron_triangle_width * 2_f64;
    let octahedron_triangle_triple_width = octahedron_triangle_width * 3_f64;
    let octahedron_triangle_double_height = octahedron_triangle_height * 2_f64;
    fs::write("octahedron-net.svg", format!(r##"<svg width="{octahedron_net_image_width}" height="{octahedron_net_image_height}" viewBox="0 0 {octahedron_net_image_width} {octahedron_net_image_height}" xmlns="http://www.w3.org/2000/svg" version="2">
    <defs>
        <g id="triangle-tile">
            <path d="M {octahedron_inner_triangle_start_x},{octahedron_inner_triangle_start_y} h {octahedron_inner_triangle_width} l -{octahedron_inner_triangle_half_width},-{octahedron_inner_triangle_height} z" fill="none" stroke="grey" stroke-width="{octahedron_triangle_narrowed_stroke_width}" />
        </g>
        <g id="rotated-triangle-tile">
            <use href="#triangle-tile" transform="rotate(180,{octahedron_triangle_half_width},{octahedron_triangle_half_height})" />
        </g>
    </defs>
    <use href="#triangle-tile" x="{octahedron_triangle_width}" y="0" />
    <use href="#rotated-triangle-tile" x="0" y="{octahedron_triangle_height}" />
    <use href="#triangle-tile" x="{octahedron_triangle_half_width}" y="{octahedron_triangle_height}" />
    <use href="#rotated-triangle-tile" x="{octahedron_triangle_width}" y="{octahedron_triangle_height}" />
    <use href="#triangle-tile" x="{octahedron_triangle_three_halves_width}" y="{octahedron_triangle_height}" />
    <use href="#rotated-triangle-tile" x="{octahedron_triangle_double_width}" y="{octahedron_triangle_height}" />
    <use href="#triangle-tile" x="{octahedron_triangle_five_halves_width}" y="{octahedron_triangle_height}" />
    <use href="#rotated-triangle-tile" x="{octahedron_triangle_three_halves_width}" y="{octahedron_triangle_double_height}" />
</svg>
"##))?;

    let octahedron_tiles_image_width_in_triangles = 4_f64;
    let octahedron_tiles_image_height_in_triangles = 2_f64;
    let octahedron_tiles_image_width = octahedron_triangle_width * octahedron_tiles_image_width_in_triangles;
    let octahedron_tiles_image_height = octahedron_triangle_height * octahedron_tiles_image_height_in_triangles;
    let octahedron_triangle_quarter_width = octahedron_triangle_width / 4_f64;
    let octahedron_wide_arc_width = 20;
    let octahedron_narrow_arc_width = 10;
    fs::write("octahedron-tiles.svg", format!(r##"<svg width="{octahedron_tiles_image_width}" height="{octahedron_tiles_image_height}" viewBox="0 0 {octahedron_tiles_image_width} {octahedron_tiles_image_height}" xmlns="http://www.w3.org/2000/svg" version="2">
    <defs>
        <path id="arc" d="M {octahedron_triangle_quarter_width},{octahedron_triangle_half_height} a {octahedron_triangle_half_width} {octahedron_triangle_half_width} 60 0 0 {octahedron_triangle_half_width},0" fill="none" />
        <g id="triangle-tile-with-arc">
            <path d="M {octahedron_inner_triangle_start_x},{octahedron_inner_triangle_start_y} h {octahedron_inner_triangle_width} l -{octahedron_inner_triangle_half_width},-{octahedron_inner_triangle_height} z" fill="none" stroke="grey" stroke-width="{octahedron_triangle_stroke_width}" />
            <use href="#arc" stroke="#e4a" stroke-width="{octahedron_wide_arc_width}" />
            <use href="#arc" stroke="#a7e" stroke-width="{octahedron_narrow_arc_width}" />
        </g>
    </defs>
    <use href="#triangle-tile-with-arc" x="0" y="0" />
    <use href="#triangle-tile-with-arc" x="{octahedron_triangle_width}" y="0" />
    <use href="#triangle-tile-with-arc" x="{octahedron_triangle_double_width}" y="0" />
    <use href="#triangle-tile-with-arc" x="{octahedron_triangle_triple_width}" y="0" />
    <use href="#triangle-tile-with-arc" x="0" y="{octahedron_triangle_height}" />
    <use href="#triangle-tile-with-arc" x="{octahedron_triangle_width}" y="{octahedron_triangle_height}" />
    <use href="#triangle-tile-with-arc" x="{octahedron_triangle_double_width}" y="{octahedron_triangle_height}" />
    <use href="#triangle-tile-with-arc" x="{octahedron_triangle_triple_width}" y="{octahedron_triangle_height}" />
</svg>
"##))?;

    Ok(())
}
