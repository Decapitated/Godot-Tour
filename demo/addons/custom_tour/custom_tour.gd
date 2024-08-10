@tool
extends EditorPlugin

var button: Button

func _enter_tree():
	var title_bar = TourSingleton.get_title_bar_full()
	
	button = Button.new()
	button.text = "Toggle Tour"
	button.pressed.connect(on_pressed)
	
	title_bar.add_child(button)
	title_bar.move_child(button, 4)

	var button_focused_node = TourSingleton.create_focused_node(button, null)
	TourSingleton.add_focused_node(button_focused_node)

	var run_bar = title_bar.find_children("*EditorRunBar*", "EditorRunBar", false, false)[0]
	if(run_bar):
		var focused_node = TourSingleton.create_focused_node(run_bar, null)
		TourSingleton.add_focused_node(focused_node)

func _exit_tree():
	# Clean-up of the plugin goes here.
	pass

func on_pressed():
	TourSingleton.theater_rect.visible = !TourSingleton.theater_rect.visible
	
func print_node(node: Node, depth = 0):
	print("%s%s: %s" % ["".lpad(depth * 4), node.name, node.get_class()])
	for child in node.get_children():
		print_node(child, depth + 1)

func get_run_bar(title_bar: Control) -> Control:
	return title_bar.find_children("*EditorRunBar*", "EditorRunBar", false, false)[0]
