shader_type canvas_item;

uniform vec4 dim_color = vec4(0, 0, 0, 0.9);
uniform vec2 rect_size = vec2(0, 0);
uniform vec2 rect_position = vec2(0, 0);

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
	bool has_point = rect_has_point(global_pos);
	COLOR = (has_point) ? TRANSPARENT : dim_color;
}

//void light() {
	// Called for every pixel for every light affecting the CanvasItem.
	// Uncomment to replace the default light processing function with this one.
//}
