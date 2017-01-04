#version 140

in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform vec2 offset_position;

//uniform mat4 matrix;

void main() {
    v_tex_coords = tex_coords;

    // offset_position is ranging from [0,0] to [1,1]
    // position is at [-1, 1] (top-left)
    // so we need to multiply the offset_position by 2, and add the x while subtracting the y
    vec2 offset_position = offset_position * vec2(2.0, 2.0);
    
    gl_Position = vec4(
        position.x + offset_position.x, 
        position.y - offset_position.y,
        0.0,
        1.0
    );
}

