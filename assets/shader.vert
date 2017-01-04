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
/*
#version 140

in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform mat3 matrix;
uniform vec2 screen_size;

void main() {
    v_tex_coords = tex_coords;
    
    mat3 matrix = matrix;
    matrix[2][0] = (matrix[2][0] / screen_size.x) * 2.0f - 1.0f;
    matrix[2][1] = 1.0f - (matrix[2][1] / screen_size.y) * 2.0f;

    vec2 position = position;
    position.x = (position.x / screen_size.x) * 2.0f - 1.0f;
    position.y = 1.0f - (position.y / screen_size.y) * 2.0f;

    vec3 actual_position = matrix * vec3(position, 1.0);
    gl_Position = vec4(actual_position.x, actual_position.y, 0.0, actual_position.z);
}
*/