// Get posiions from vertex shader
in vec4 pos;
// Color output
varying out vec4 color;

struct complexNum {
  float real;
  float imag;
};

bool outOfBound(complexNum vert) {
  float sqr = (vert.real * vert.real) + (vert.imag * vert.imag);
  return (sqr > 4.0);
}

complexNum compMul(complexNum first, complexNum sec) {
  complexNum ret = complexNum(0.0,0.0);

  ret.real = (first.real * sec.real) - (first.imag * sec.imag);
  ret.imag = (first.imag * sec.real) + (first.real * sec.imag);

  return ret;
}

complexNum compAdd(complexNum first, complexNum sec){
  return complexNum(first.real + sec.real, first.imag + sec.imag);
}

int mandel(complexNum position) {
  complexNum comp1 = complexNum(0.0, 0.0);
  complexNum comp2 = complexNum(0.0, 0.0);
  int i;

  for (i = 0; i < 1000; i++) {
    comp2 = compAdd(compMul(comp1, comp1), position);

    if (outOfBound(comp2)) break;

    comp1 = comp2;
    comp2 = complexNum(0.0, 0.0);
  }

  return i;
}

void main() {
  int n = mandel((complexNum(pos[0],pos[1])));
  if (n < 200) color = vec4( 1.0-(n)/200.0, 1.0-((n+789)%200)/200.0, (n)/200.0, 1.0);
  else color = vec4(0.0, 0.0, 0.0, 1.0);
}
