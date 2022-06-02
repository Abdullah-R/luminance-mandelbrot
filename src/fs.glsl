// Get posiions from vertex shader
in vec4 pos;
// Color output
varying out vec4 color;

bool outOfBound(vec2 vert) {
  float sqr = (vert[0] * vert[0]) + (vert[1] * vert[1]);
  return (sqr > 4.0);
}

vec2 compMul(vec2 first, vec2 sec) {
  vec2 ret;

  ret[0] = (first[0] * sec[0]) - (first[1] * sec[1]);
  ret[1] = (first[1] * sec[0]) + (first[0] * sec[1]);

  return ret;
}

int mandel(vec2 position) {
  vec2 comp1 = vec2(0.0, 0.0);
  vec2 comp2 = vec2(0.0, 0.0);
  int i;

  for (i = 0; i < 200; i++) {
    comp2 = compMul(comp1, comp1) + position;

    if (outOfBound(comp2)) break;

    comp1 = comp2;
    comp2 = vec2(0.0, 0.0);
  }

  return i;
}

void main() {
  int n = mandel(pos);
  if (n < 200) color = vec4(1.0-((n+129)%200)/200.0, 1.0-((n+93)%200)/200.0, 1.0-((n+100)%200)/200.0, 1.0);
  else color = vec4(0.0, 0.0, 0.0, 1.0);
}
