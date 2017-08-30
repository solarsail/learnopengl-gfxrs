#version 330 core
out vec4 FragColor;

uniform Lighting {
    vec3 objectColor;
    vec3 lightColor;
};

void main()
{
    FragColor = vec4(lightColor * objectColor, 1.0);
}
