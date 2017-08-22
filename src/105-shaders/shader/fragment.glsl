#version 330 core
in vec3 ourColor;
out vec4 FragColor;

uniform Modifier {
    vec3 colorMod;
};

void main()
{
    FragColor = vec4(ourColor * colorMod, 1.0);
}
