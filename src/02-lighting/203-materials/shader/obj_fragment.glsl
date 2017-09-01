#version 330 core
out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;

uniform Lighting {
    vec3 objectColor;
    vec3 lightColor;
    vec3 lightPos;
    vec3 viewPos;
};

struct Material {
    vec3 ambient;
    float shininess;
    vec3 diffuse;
    vec3 specular;
};

uniform MaterialDef {
    Material material;
};

void main()
{
    vec3 ambient = material.ambient * lightColor;

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * material.diffuse * lightColor;

    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = spec * material.specular * lightColor;

    vec3 result = (ambient + diffuse + specular) * objectColor;
    FragColor = vec4(result, 1.0);
}
