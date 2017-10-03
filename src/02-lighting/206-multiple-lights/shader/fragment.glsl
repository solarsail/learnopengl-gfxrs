#version 330 core
out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;
in vec2 TexCoords;

struct DirLight {
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    vec4 dir;
};

struct PointLight {
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    vec4 pos;
    float a0, a1, a2, pad;
};

uniform u_dirLights {
    DirLight dirLights[16];
};

uniform u_pointLights {
    PointLight pointLights[64];
};

uniform u_lightArgs {
    int num_dir;
    int num_point;
};

uniform float material_shininess;
uniform sampler2D material_diffuse;
uniform sampler2D material_specular;
uniform vec3 viewPos;

vec4 CalcDirLight(DirLight light, vec4 normal, vec4 viewDir)
{
    vec4 lightDir = normalize(-light.dir);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    vec4 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material_shininess);
    // combine results
    vec4 ambient  = light.ambient  * texture(material_diffuse, TexCoords);
    vec4 diffuse  = light.diffuse  * diff * texture(material_diffuse, TexCoords);
    vec4 specular = light.specular * spec * texture(material_specular, TexCoords);
    return (ambient + diffuse + specular);
}

vec4 CalcPointLight(PointLight light, vec4 normal, vec4 fragPos, vec4 viewDir)
{
    vec4 lightDir = normalize(light.pos - fragPos);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    vec4 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material_shininess);
    // attenuation
    float distance    = length(light.pos - fragPos);
    float attenuation = 1.0 / (light.a0 + light.a1 * distance + light.a2 * (distance * distance));    
    // combine results
    vec4 ambient  = light.ambient  * texture(material_diffuse, TexCoords);
    vec4 diffuse  = light.diffuse  * diff * texture(material_diffuse, TexCoords);
    vec4 specular = light.specular * spec * texture(material_specular, TexCoords);
    ambient  *= attenuation;
    diffuse  *= attenuation;
    specular *= attenuation;
    return (ambient + diffuse + specular);
}

void main()
{
    // properties
    vec4 norm = vec4(normalize(Normal), 0.0);
    vec4 viewDir = vec4(normalize(viewPos - FragPos), 0.0);

    vec4 result = vec4(0.0);
    // phase 1: Directional lighting
    for(int i = 0; i < num_dir; i++)
        result = CalcDirLight(dirLights[i], norm, viewDir);
    // phase 2: Point lights
    for(int i = 0; i < num_point; i++)
        result += CalcPointLight(pointLights[i], norm, vec4(FragPos, 1.0), viewDir);    
    // phase 3: Spot light
    //result += CalcSpotLight(spotLight, norm, FragPos, viewDir);    
    
    FragColor = result;
}
