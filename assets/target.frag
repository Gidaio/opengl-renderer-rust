#version 330 core
struct Material {
    sampler2D diffuseColor;
    sampler2D specularColor;
    float shininess;
};

struct Light {
    vec3 position;
    vec3 direction;
    float innerCutoff;
    float outerCutoff;

    vec3 ambientColor;
    vec3 diffuseColor;
    vec3 specularColor;

    float constant;
    float linear;
    float quadratic;
};

in vec3 FragmentPosition;
in vec3 Normal;
in vec2 TextureCoordinate;

out vec4 FragColor;

uniform Material material;
uniform Light light;
uniform vec3 viewerPosition;

void main()
{
    vec3 lightDirection = normalize(light.position - FragmentPosition);
    float theta = dot(lightDirection, normalize(-light.direction));
    float epsilon = light.innerCutoff - light.outerCutoff;
    float intensity = clamp((theta - light.outerCutoff) / epsilon, 0.0, 1.0);

    float distanceToLight = length(light.position - FragmentPosition);
    float attenuation = 1.0 / (light.constant + light.linear * distanceToLight + light.quadratic * distanceToLight * distanceToLight);

    vec3 ambientLight = vec3(texture(material.diffuseColor, TextureCoordinate)) * light.ambientColor;
    ambientLight *= attenuation;

    vec3 normal = normalize(Normal);
    float diffuseStrength = max(dot(normal, lightDirection), 0.0);
    vec3 diffuseLight = diffuseStrength * vec3(texture(material.diffuseColor, TextureCoordinate)) * light.diffuseColor;
    diffuseLight *= attenuation * intensity;

    vec3 viewDirection = normalize(viewerPosition - FragmentPosition);
    vec3 reflectionDirection = reflect(-lightDirection, normal);
    float specularStrength = pow(max(dot(viewDirection, reflectionDirection), 0.0), material.shininess);
    vec3 specularLight = specularStrength * vec3(texture(material.specularColor, TextureCoordinate)) * light.specularColor;
    specularLight *= attenuation * intensity;

    vec3 totalLight = ambientLight + diffuseLight + specularLight;
    FragColor = vec4(totalLight, 1.0);
}
