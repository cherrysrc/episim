// Simple default vertex shader to display entities
#version 430
layout (location = 0) in vec3 vPos;
layout (location = 1) in vec3 vColor;
layout (location = 2) in vec2 vTexCoord;

out vec3 outColor;

void main(){
    gl_Position = vec4(vPos.x, vPos.y, vPos.z, 1.0);
    outColor = vColor;
}