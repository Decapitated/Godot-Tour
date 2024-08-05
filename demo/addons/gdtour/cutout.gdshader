shader_type canvas_item;

uniform vec2 rect_size = vec2(0, 0);
uniform vec2 rect_position = vec2(0, 0);

uniform float corner_radius = 0;
uniform float border_width = 0;
uniform vec4 background_color = vec4(0, 0, 0, 0.9);
uniform vec4 border_color = vec4(1, 0, 0, 1);

vec2 get_rect_center() {
	return rect_position + rect_size/2.0;
}

vec2 get_inner_rect_corner() {
	return rect_position + rect_size - corner_radius;
}

float get_distance_from_rect(vec2 point, vec2 rect_corner) {
	vec2 rect_center = get_rect_center();
	vec2 point_vec = point - rect_center;
	vec2 corner_vec = rect_corner - rect_center;
	vec2 q = abs(point_vec) - corner_vec;
	return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - corner_radius;
}

bool rect_has_point(vec2 point) {
	vec2 bottom_right_corner = rect_position + rect_size;
	return (rect_position.x <= point.x && point.x <= bottom_right_corner.x) &&
		   (rect_position.y <= point.y && point.y <= bottom_right_corner.y);
}

varying vec2 global_pos;
void vertex() {
	vec4 world_pos = MODEL_MATRIX * vec4(VERTEX, 0.0, 0.0);
	global_pos = world_pos.xy;
}

const vec4 TRANSPARENT = vec4(0);
void fragment() {
	float inner_rect_distance = get_distance_from_rect(global_pos, get_inner_rect_corner());
	float outer_rect_distance = get_distance_from_rect(global_pos, get_inner_rect_corner() + border_width);
	
	vec4 bg_color = background_color;
	bg_color.a *= smoothstep(0.1, 0.75, outer_rect_distance);
	
	vec4 b_color = border_color;
	b_color.a *= smoothstep(0.1, 0.75, inner_rect_distance);
	
	COLOR = mix(b_color, bg_color, smoothstep(0.0, 1.0, outer_rect_distance));
}

//void light() {
	// Called for every pixel for every light affecting the CanvasItem.
	// Uncomment to replace the default light processing function with this one.
//}
