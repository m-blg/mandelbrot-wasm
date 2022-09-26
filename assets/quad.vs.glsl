const uint ind_a[6] = uint[6](
    0u, 1u, 3u, 1u, 2u, 3u
);

const vec2 pos_a[4] = vec2[4](
    vec2(-1.0f, -1.0f),
    vec2(1.0f, -1.0f),
    vec2(1.0f, 1.0f),
    vec2(-1.0f, 1.0f)
);
const vec2 uv_a[4] = vec2[4](
    vec2(0.0f, 0.0f),
    vec2(1.0f, 0.0f),
    vec2(1.0f, 1.0f),
    vec2(0.0f, 1.0f)
);

out vec2 pos;
out vec2 uv;


void main() {
    uint ind = ind_a[gl_VertexID];
    pos = pos_a[ind];
    uv = uv_a[ind];
    gl_Position = vec4(pos, 0.0, 1.0);
}