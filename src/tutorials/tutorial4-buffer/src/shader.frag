#version 450

layout(binding = 0) uniform Input {
    vec2 u_resolution;
    vec2 u_mouse;
    float u_time;
};

layout(location=0) in vec3 v_color;
layout(location=0) out vec4 f_color;

float is_inside(vec2 xy, float radius) {
  return step(length(xy), radius);
}

vec3 draw_circle(vec2 coord, float radius, vec2 mouse) {
  vec2 st = gl_FragCoord.xy/u_resolution.xy;
  st.x *= u_resolution.x/u_resolution.y;
    
  vec3 color = vec3(is_inside(coord, radius));
  color = vec3(st.x * color.x, st.y * color.y, abs(sin(u_time)) * color.z);
  //color.xyz = is_inside(mouse, radius) == 1.0 ? vec3(1.0 * color.x * abs(sin(u_time)), 0, 0) : color.xyz;
  return color;
}

void main() {
  vec2 coord = v_color.xy / u_resolution;
  vec2 mouse = u_mouse.xy / u_resolution;
  float radius = 0.3;
  vec2 offset = vec2(0.5, 0.5);
  vec3 color = draw_circle(coord - offset, radius, mouse - offset);

  f_color = vec4(color, 1.0);
}

