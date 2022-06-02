// Position of each corner of the screen
in vec2 position;

// Output position to the fragment shader
varying out vec4 pos;

//Produce vertex positions for hardware
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    pos = vec4(1.5*position[0] - 0.6, 1.3*position[1], 0.0, 1.0);
}