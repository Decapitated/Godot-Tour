@tool
extends EditorPlugin

var button: Button

func _enter_tree():
	button = Button.new()
	button.text = "Toggle Tour"
	button.pressed.connect(on_pressed)
	
	var title_bar = Tour.get_title_bar_full()
	title_bar.add_child(button)
	title_bar.move_child(button, 4)

	var button_focused_node = Tour.create_focused_node(button, null)
	Tour.add_focused_node(button_focused_node)

	var run_bar = Tour.get_run_bar(title_bar)
	var focused_node = Tour.create_focused_node(run_bar, null)
	Tour.add_focused_node(focused_node)

func _exit_tree():
	# Clean-up of the plugin goes here.
	pass

func on_pressed():
	Tour.theater_rect.visible = !Tour.theater_rect.visible
