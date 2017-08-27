#version 330 core
out vec4 FragColor;

in vec2 TexCoord;


void main()
{
    FragColor = mix(texture(ourTexture1, TexCoord), texture(ourTexture2, TexCoord), 0.2);
}
