#version 330 core
out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;
in vec3 LightPos;

uniform Light {
    vec3 light_ambient;
    vec3 light_diffuse;
    vec3 light_specular;
    vec3 light_pos;
};

uniform Material {
    vec3 material_ambient;
    float material_shininess;
    vec3 material_diffuse;
    vec3 material_specular;
};

void main()
{
    vec3 ambient = material_ambient * light_ambient;

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(LightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * material_diffuse * light_diffuse;

    vec3 viewDir = normalize(-FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material_shininess);
    vec3 specular = spec * material_specular * light_specular;

    vec3 result = (ambient + diffuse + specular);
    FragColor = vec4(result, 1.0);
}
