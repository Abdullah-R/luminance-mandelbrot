// Position of each corner of the screen
in vec2 position;

uniform vec3 trans;

// Output position to the fragment shader
varying out vec4 pos;

vec2 transform(vec2 orig) {
    return vec2 ((trans[2]*(1.5*orig[0]) - 0.6 + trans[0]), (trans[2]*(1.3*orig[1]) - trans[1]));
}


//Produce vertex positions for hardware
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    pos = vec4(transform(position), 0.0, 1.0); // transform pixel posiition to complex plane position
}