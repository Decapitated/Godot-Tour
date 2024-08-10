@tool
extends EditorPlugin

var button: Button

func _enter_tree():
	var title_bar = TourSingleton.get_title_bar_full()
	
	button = Button.new()
	button.text = "Toggle Tour"
	button.pressed.connect(on_pressed)
	
	var run_bar = TourSingleton.get_run_bar(title_bar)
	var focused_node = TourSingleton.create_focused_node(run_bar, null)
	TourSingleton.add_focused_node(focused_node)
	
	title_bar.add_child(button)
	title_bar.move_child(button, 4)

	var button_focused_node = TourSingleton.create_focused_node(button, null)
	TourSingleton.add_focused_node(button_focused_node)
	

func _exit_tree():
	# Clean-up of the plugin goes here.
	pass

func on_pressed():
	TourSingleton.theater_rect.visible = !TourSingleton.theater_rect.visible
