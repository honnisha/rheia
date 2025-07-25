shader_type spatial;

render_mode blend_mix, depth_prepass_alpha;

uniform vec3 character_position = vec3(7.5, 0.0, 7.5);
uniform float fade_distance = 200.0;
uniform float fade_range = 8.0;
const vec4 color = vec4(1.0, 1.0, 1.0, 1.0);

varying vec3 world_vertex;

uniform sampler2D texture_albedo : source_color, filter_nearest;

void vertex() {
	world_vertex = (MODEL_MATRIX * vec4(VERTEX, 1.0)).xyz;
}

void fragment() {
	float chardistance = length(character_position - world_vertex);
	float alpha = 1.0 - smoothstep(fade_distance, fade_distance + fade_range, chardistance);

    vec4 albedo_tex = texture(texture_albedo, UV);
    ALBEDO = albedo_tex.rgb;
    ALPHA = albedo_tex.a * alpha;
}