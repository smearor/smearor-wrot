// Color mask shader for DMA-BUF textures
// This shader removes pixels that match the mask color within a threshold

uniform float threshold;
uniform vec3 mask_color;

void main() {
    // u_texture[0] is the standard input from GTK for the first node
    vec4 tex_color = GskTexture(u_texture[0], gsk_get_tex_coord(0));

    // Calculate distance between pixel color and mask color
    float d = distance(tex_color.rgb, mask_color);

    if (d < threshold) {
        // Make pixel transparent (mask it out)
        gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
    } else {
        gl_FragColor = tex_color;
    }
}
