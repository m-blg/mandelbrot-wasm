// #extension GL_ARB_shader_bit_encoding : enable
precision highp float;

in vec2 pos;
in vec2 uv;

out vec4 color;

uniform vec2 u_trans;
uniform vec2 u_scale;
uniform float[6] u_color_coefs;
uniform float u_R;
uniform int u_MAX_ITER;

vec2
complex_mul(vec2 lhs, vec2 rhs) {
    return vec2(lhs.x*rhs.x - lhs.y*rhs.y, lhs.x*rhs.y + lhs.y*rhs.x);
}

// dvec2
// complex_mul_d(dvec2 lhs, dvec2 rhs) {
//     return dvec2(lhs.x*rhs.x - lhs.y*rhs.y, lhs.x*rhs.y + lhs.y*rhs.x);
// }

vec3
random_color(float t)
{
    float x = fract(sin(u_color_coefs[0]*t) + 0.01* u_color_coefs[3]); 
    float y = fract(sin(u_color_coefs[1]*x) + 0.01* u_color_coefs[4]); 
    float z = fract(sin(u_color_coefs[2] * (y + x)) + 0.01* u_color_coefs[5]); 
    return vec3(x, y, z);
}

float
mandelbrot(vec2 p, float R, int MAX_ITER) 
{
    vec2 c = p;
    vec2 z = vec2(0.0);

    for (int i = 1; i < MAX_ITER; i++)
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
    //vec2 mpos = vec2(u_mouse_pos.x-0.5, -(u_mouse_pos.y-0.5));
    vec2 p = vec2(-0.5) + uv;
    p /= vec2(u_scale);
    p += vec2(u_trans);
    float m = mandelbrot(p, u_R, u_MAX_ITER);
    color = vec4(random_color(m), 1.0);
    // color = vec4(vec3(m, 1.0);
}
