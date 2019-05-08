#version 330 core

in float magnitude;
in float range;

out vec4 Color;

void main() {
    Color = vec4(magnitude/range, 0.0, 0.0, 1.0);
}