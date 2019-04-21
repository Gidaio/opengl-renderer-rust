#version 330 core
in vec3 FragmentPosition;
in vec3 Normal;

out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPosition;
uniform vec3 viewerPosition;

void main()
{
    vec3 normal = normalize(Normal);
    vec3 lightDirection = normalize(lightPosition - FragmentPosition);
    vec3 viewDirection = normalize(viewerPosition - FragmentPosition);
    vec3 reflectionDirection = reflect(-lightDirection, normal);

    float specularStrength = 0.5;
    float specular = pow(max(dot(viewDirection, reflectionDirection), 0.0), 32);
    vec3 specularLight = specularStrength * specular * lightColor;

    float diffuseStrength = max(dot(normal, lightDirection), 0.0);
    vec3 diffuseLight = diffuseStrength * lightColor;

    float ambientStrength = 0.1;
    vec3 ambientLight = ambientStrength * lightColor;

    vec3 totalLight = (ambientLight + diffuseLight + specularLight) * objectColor;
    FragColor = vec4(totalLight, 1.0);
}
