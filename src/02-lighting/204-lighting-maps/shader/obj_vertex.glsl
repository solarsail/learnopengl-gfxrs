#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoord;

out vec3 Normal;
out vec3 FragPos;
out vec3 LightPos;
out vec2 TexCoord;

uniform Transform {
    mat4 model;
    mat4 view;
    mat4 projection;
};

uniform Light {
    vec3 light_ambient;
    vec3 light_diffuse;
    vec3 light_specular;
    vec3 light_pos;
};

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    FragPos = vec3(view * model * vec4(aPos, 1.0));
    LightPos = vec3(view * model * vec4(light_pos, 1.0));
    Normal = mat3(transpose(inverse(view * model))) * aNormal;
    TexCoord = aTexCoord;
}
