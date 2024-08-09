shader_type canvas_item;

uniform vec4 rects[1000];
uniform int corners[1000];

uniform vec4 background_color = vec4(0, 0, 0, 0.9);

vec2 get_rect_position(vec4 rect) {
	return vec2(rect.x, rect.y);
}

vec2 get_rect_size(vec4 rect) {
	return vec2(rect.z, rect.w);
}

vec2 get_rect_center(vec4 rect) {
	return get_rect_position(rect) + get_rect_size(rect)/2.0;
}

vec2 get_inner_rect_corner(vec4 rect, float corner_radius) {
	return get_rect_position(rect) + get_rect_size(rect) - min(min(corner_radius, rect.z/2.0), rect.w/2.0);
}

float get_distance_from_rect(vec4 rect, vec2 point, vec2 rect_corner, float corner_radius) {
	vec2 rect_center = get_rect_center(rect);
	vec2 point_vec = point - rect_center;
	vec2 corner_vec = rect_corner - rect_center;
	vec2 q = abs(point_vec) - corner_vec;
	return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - min(min(corner_radius, rect.z/2.0), rect.w/2.0);
}

float get_min_dist_from_rects(vec2 point) {
	float min_distance = get_distance_from_rect(rects[0], point, get_inner_rect_corner(rects[0], float(corners[0])), float(corners[0]));
	for(int i = 1; i < rects.length(); i++) {
		if(rects[i] == vec4(0.0)) continue;
		min_distance = min(min_distance, get_distance_from_rect(rects[i], point, get_inner_rect_corner(rects[i], float(corners[i])), float(corners[i])));
	}
	return min_distance;
}

varying vec2 global_pos;
void vertex() {
	vec4 world_pos = MODEL_MATRIX * vec4(VERTEX, 0.0, 0.0);
	global_pos = world_pos.xy;
}

const vec4 TRANSPARENT = vec4(0);
void fragment() {
	float inner_rect_distance = get_min_dist_from_rects(global_pos);
	
	vec4 bg_color = background_color;
	bg_color.a *= smoothstep(0.1, 0.75, inner_rect_distance);
	
	COLOR = bg_color;
}

//void light() {
	// Called for every pixel for every light affecting the CanvasItem.
	// Uncomment to replace the default light processing function with this one.
//}
