#version 330 core
in vec3 FragmentPosition;
in vec3 Normal;

out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform vec3 lightPosition;

void main()
{
    vec3 normal = normalize(Normal);
    vec3 lightDirection = normalize(lightPosition - FragmentPosition);

    float diffuseStrength = max(dot(normal, lightDirection), 0.0);
    vec3 diffuseLight = lightColor * diffuseStrength;

    float ambientStrength = 0.1;
    vec3 ambientLight = lightColor * ambientStrength;

    vec3 totalLight = objectColor * (diffuseLight);
    FragColor = vec4(totalLight, 1.0);
}
