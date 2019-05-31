#version 330 core
struct Material {
    vec3 diffuseColor;
    vec3 specularColor;
    vec3 emissiveColor;
    float shininess;
};

struct LightColors {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct LightAttenuation {
    float constant;
    float linear;
    float quadratic;
};

struct DirectionalLight {
    vec3 direction;
    LightColors colors;
};

struct PointLight {
    vec3 position;
    LightColors colors;
    LightAttenuation attenuation;
};

struct Spotlight {
    vec3 position;
    vec3 direction;
    float innerCutoff;
    float outerCutoff;

    LightColors colors;
    LightAttenuation attenuation;
};

in vec3 FragmentPosition;
in vec3 Normal;
in vec2 TextureCoordinate;

out vec4 FragColor;

uniform Material material;
uniform DirectionalLight directionalLight;
#define POINT_LIGHT_MAX 4
uniform PointLight pointLights[POINT_LIGHT_MAX];
uniform Spotlight spotlight;
uniform vec3 viewerPosition;


vec3 calculateDirectionalLight(DirectionalLight light, vec3 normal, vec3 viewDirection);
vec3 calculatePointLight(PointLight light, vec3 normal, vec3 viewDirection);
vec3 calculateSpotlight(Spotlight light, vec3 normal, vec3 viewDirection);


void main()
{
    vec3 normal = normalize(Normal);
    vec3 viewDirection = normalize(viewerPosition - FragmentPosition);

    vec3 totalLight = calculateDirectionalLight(directionalLight, normal, viewDirection);

    for (int i = 0; i < POINT_LIGHT_MAX; i++) {
        totalLight += calculatePointLight(pointLights[i], normal, viewDirection);
    }

    totalLight += calculateSpotlight(spotlight, normal, viewDirection);
    totalLight += material.emissiveColor;

    FragColor = vec4(totalLight, 1.0);
}


vec3 calculateDirectionalLight(DirectionalLight light, vec3 normal, vec3 viewDirection) {
    vec3 ambientLight = material.diffuseColor * light.colors.ambient;

    vec3 lightDirection = normalize(-light.direction);
    float diffuseStrength = max(dot(normal, lightDirection), 0.0);
    vec3 diffuseLight = diffuseStrength * material.diffuseColor * light.colors.diffuse;

    vec3 reflectionDirection = reflect(-lightDirection, normal);
    float specularStrength = pow(max(dot(viewDirection, reflectionDirection), 0.0), material.shininess);
    vec3 specularLight = specularStrength * material.specularColor * light.colors.specular;

    return ambientLight + diffuseLight + specularLight;
}


vec3 calculatePointLight(PointLight light, vec3 normal, vec3 viewDirection) {
    float distanceToLight = length(light.position - FragmentPosition);
    float attenuation = 1.0 / (
        light.attenuation.constant +
        light.attenuation.linear * distanceToLight +
        light.attenuation.quadratic * distanceToLight * distanceToLight
    );

    vec3 ambientLight = material.diffuseColor * light.colors.ambient;
    ambientLight *= attenuation;

    vec3 lightDirection = normalize(light.position - FragmentPosition);
    float diffuseStrength = max(dot(normal, lightDirection), 0.0);
    vec3 diffuseLight = diffuseStrength * material.diffuseColor * light.colors.diffuse;
    diffuseLight *= attenuation;

    vec3 reflectionDirection = reflect(-lightDirection, normal);
    float specularStrength = pow(max(dot(viewDirection, reflectionDirection), 0.0), material.shininess);
    vec3 specularLight = specularStrength * material.specularColor * light.colors.specular;
    specularLight *= attenuation;

    return ambientLight + diffuseLight + specularLight;
}


vec3 calculateSpotlight(Spotlight light, vec3 normal, vec3 viewDirection) {
    vec3 lightDirection = normalize(light.position - FragmentPosition);
    float theta = dot(lightDirection, normalize(-light.direction));
    float epsilon = light.innerCutoff - light.outerCutoff;
    float intensity = clamp((theta - light.outerCutoff) / epsilon, 0.0, 1.0);

    float distanceToLight = length(light.position - FragmentPosition);
    float attenuation = 1.0 / (
        light.attenuation.constant +
        light.attenuation.linear * distanceToLight +
        light.attenuation.quadratic * distanceToLight * distanceToLight
    );

    vec3 ambientLight = material.diffuseColor * light.colors.ambient;
    ambientLight *= attenuation;

    float diffuseStrength = max(dot(normal, lightDirection), 0.0);
    vec3 diffuseLight = diffuseStrength * material.diffuseColor * light.colors.diffuse;
    diffuseLight *= attenuation * intensity;

    vec3 reflectionDirection = reflect(-lightDirection, normal);
    float specularStrength = pow(max(dot(viewDirection, reflectionDirection), 0.0), material.shininess);
    vec3 specularLight = specularStrength * material.specularColor * light.colors.specular;
    specularLight *= attenuation * intensity;

    return ambientLight + diffuseLight + specularLight;
}
