pub const TORCH_FRAGMENT_SHADER: &str = r#"
#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

uniform sampler2D texture0;
uniform vec2 playerPos;
uniform vec2 resolution;
uniform float lightRadius;
uniform float lightIntensity;

out vec4 finalColor;

void main()
{
    vec4 texColor = texture(texture0, fragTexCoord);

    // Convert fragment position to screen coordinates
    vec2 fragPos = fragTexCoord * resolution;

    // Calculate distance from fragment to player
    float dist = distance(fragPos, playerPos);

    // Create smooth falloff
    float falloff = smoothstep(lightRadius, 0.0, dist);

    // Apply light intensity with some ambient light
    float ambient = 0.05; // Small amount of ambient light
    float light = mix(ambient, lightIntensity, falloff);

    // Apply lighting to texture
    finalColor = vec4(texColor.rgb * light, texColor.a);
}
"#;
