#version 330 core
out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;
in vec3 LightPos;
in vec2 TexCoord;

uniform Light {
    vec3 light_ambient;
    vec3 light_diffuse;
    vec3 light_specular;
    vec3 light_pos;
};

uniform Material {
    float material_shininess;
};

uniform sampler2D material_diffuse;
uniform sampler2D material_specular;

void main()
{
    vec3 ambient = light_ambient * vec3(texture(material_diffuse, TexCoord));

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(LightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * light_diffuse * vec3(texture(material_diffuse, TexCoord));

    vec3 viewDir = normalize(-FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material_shininess);
    vec3 specular = spec * light_specular * vec3(texture(material_specular, TexCoord));

    vec3 result = (ambient + diffuse + specular);
    FragColor = vec4(result, 1.0);
}
