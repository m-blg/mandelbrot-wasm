precision mediump float;

in vec2 pos;
in vec2 uv;

out vec4 color;

uniform vec2 traslation;
uniform vec2 scale;

vec2
complex_mul(vec2 lhs, vec2 rhs) {
    return vec2(lhs.x*rhs.x - lhs.y*rhs.y, lhs.x*rhs.y + lhs.y*rhs.x);
}

float
mandelbrot(vec2 p) 
{
    vec2 c = p;
    vec2 z = vec2(0.0);

    const int MAX_ITER = 128;
    const float R = 2.0;

    for (int i = 0; i < MAX_ITER; i++)
    {
        z = complex_mul(z, z) + c;
        if (dot(z,z) > R) {
            return float(i)/float(MAX_ITER);
        }
    }

    return 0.0;
}

void 
main() {
    uv += traslation;
    uv *= scale;
    float m = mandelbrot(uv);
    color = vec4(vec3(m), 1.0);
}