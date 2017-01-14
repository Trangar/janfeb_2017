#version 140
 
in vec2 position;

uniform vec4 color;
out vec4 v_color;

uniform mat3 matrix;
uniform vec2 screen_size;
 
void main() {
    v_color = color;

    vec3 actual_position = matrix * vec3(position, 1.0);
    gl_Position = vec4((actual_position.x/screen_size.x)*2.0 - 1.0, 1.0 - (actual_position.y/screen_size.y)*2.0, 0.0, 1.0);
}
