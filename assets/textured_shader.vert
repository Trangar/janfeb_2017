#version 140
 
in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;
 
uniform mat3 matrix;
uniform vec2 screen_size;
 
void main() {
    v_tex_coords = tex_coords;
    vec3 actual_position = matrix * vec3(position, 1.0);
    gl_Position = vec4((actual_position.x/screen_size.x)*2.0 - 1.0, 1.0 - (actual_position.y/screen_size.y)*2.0, 0.0, 1.0);
}