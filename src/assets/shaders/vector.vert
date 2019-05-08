#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in float in_magnitude;
uniform float in_range;

out float magnitude;
out float range;
void main() {
    gl_Position = vec4(Position, 1.0);
    magnitude = in_magnitude;
    range = in_range;
}