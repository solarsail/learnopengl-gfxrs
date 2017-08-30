#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D ourTexture1;
uniform sampler2D ourTexture2;

void main()
{
    vec4 rgba_texture1 = texture(ourTexture1, TexCoord);
    vec4 rgba_texture2 = texture(ourTexture2, TexCoord);
    float mix_value = rgba_texture2.a * 0.2;

    FragColor = mix(rgba_texture1, rgba_texture2, mix_value) * vec4(ourColor, 1.0);
}
