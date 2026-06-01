uniform sampler2D u_texture1;
uniform float threshold;
uniform vec3 mask_color;

void main() {
    vec4 tex_color = GskTexture(u_texture1[0], gsk_get_tex_coord(0));
    gsk_set_output_color(vec4(0.0, 1.0, 0.0, 1.0));
/*

    if (mask_color.r > 2.0) {
        gsk_set_output_color(vec4(1.0, 0.0, 0.0, 1.0)); // Red for garbage data
    } else {
        gsk_set_output_color(vec4(0.0, 0.0, 1.0, 1.0)); // Blue for ok data
    }

    float d = distance(tex_color.rgb, mask_color);
    if (d < threshold) {
        gsk_set_output_color(vec4(0.0, 0.0, 0.0, 0.0));
    } else {
        gsk_set_output_color(tex_color);
    }
*/
}
