#version 450

const vec2 positions[3] = vec2[3] (
    vec2(0.0, 0.5),
    vec2(-0.5, -0.5),
    vec2(0.5, -0.5)
);

layout(location=1) out vec4 special_color;

void main() {
    special_color = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    gl_Position = special_color;
}
