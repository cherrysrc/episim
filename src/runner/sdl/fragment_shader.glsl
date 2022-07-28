// Simple default fragment shader to display entities
#version 430
out vec4 FragColor;
in vec3 outColor;

void main() {
    FragColor = vec4(outColor, 1.0);
}