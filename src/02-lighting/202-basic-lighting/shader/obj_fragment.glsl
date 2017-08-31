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

void main()
{
    float ambientStrength = 0.1;
    float specularStrength = 0.5;

    vec3 ambient = ambientStrength * lightColor;

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;

    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = (ambient + diffuse + specular) * objectColor;
    FragColor = vec4(result, 1.0);
}
