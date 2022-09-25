precision mediump float;
in vec2 pos;
out vec4 color;
void main() {
    color = vec4(pos, 0.5, 1.0);
}