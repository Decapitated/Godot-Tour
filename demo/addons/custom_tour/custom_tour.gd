@tool
extends EditorPlugin

var TheaterTheme: Theme = preload("res://TheaterRect-Editor.theme");

var toggle_button: Button
var reset_button: Button

func _enter_tree():
	toggle_button = Button.new()
	toggle_button.text = "Toggle Tour"
	toggle_button.pressed.connect(on_toggle_pressed)
	
	reset_button = Button.new()
	reset_button.text = "Reset Tour"
	reset_button.pressed.connect(on_reset_pressed)
	
	var title_bar = Tour.get_title_bar_full()
	title_bar.add_child(toggle_button)
	title_bar.move_child(toggle_button, 4)
	title_bar.add_child(reset_button)
	title_bar.move_child(reset_button, 4)
	
	setup()

func _exit_tree():
	# Clean-up of the plugin goes here.
	pass
	
func setup():
	Tour.theater_rect.theme = TheaterTheme
	Tour.theater_rect.confine_input = true
	
	var editor_interface = get_editor_interface()
	
	var base_control = Tour.get_base_control()
	
	var title_bar = Tour.get_title_bar(base_control)
	var run_bar = Tour.get_run_bar(title_bar)
	
	var main_control = Tour.get_main(base_control)
	var scene_tree_dock = Tour.get_scene_tree_dock(main_control)
	
	#region Toggle Button
	
	var toggle_button_overlay = Panel.new()
	toggle_button_overlay.mouse_filter = Control.MOUSE_FILTER_IGNORE
	Tour.theater_rect.add_child(toggle_button_overlay)
	
	var toggle_button_focused_node = Tour.create_focused_node(toggle_button, toggle_button_overlay)
	Tour.add_focused_node(toggle_button_focused_node)
	
	#endregion
	#region Run Bar
	
	var run_bar_overlay = Panel.new()
	run_bar_overlay.mouse_filter = Control.MOUSE_FILTER_IGNORE
	Tour.theater_rect.add_child(run_bar_overlay)

	var run_bar_focused_node = Tour.create_focused_node(run_bar, run_bar_overlay)
	Tour.add_focused_node(run_bar_focused_node)
	
	#endregion
	#region Scene Tree
	
	var scene_tree_overlay = Panel.new()
	scene_tree_overlay.mouse_filter = Control.MOUSE_FILTER_IGNORE
	Tour.theater_rect.add_child(scene_tree_overlay)
	
	var scene_tree = scene_tree_dock.find_child("*SceneTreeEditor*", false, false)
	var scene_tree_focused_node = Tour.create_focused_node(scene_tree, scene_tree_overlay)
	
	Tour.add_focused_node(scene_tree_focused_node)
	
	#endregion
	#region Main Screen
	
	var main_screen_overlay = Panel.new()
	main_screen_overlay.mouse_filter = Control.MOUSE_FILTER_IGNORE
	Tour.theater_rect.add_child(main_screen_overlay)
	
	var main_screen = editor_interface.get_editor_main_screen()
	var main_screen_focused_node = Tour.create_focused_node(main_screen, main_screen_overlay)
	
	Tour.add_focused_node(main_screen_focused_node)
	
	#endregion

func clean():
	Tour.clear_focused_nodes()
	for child in Tour.theater_rect.get_children():
		child.free()

func on_toggle_pressed():
	Tour.theater_rect.visible = !Tour.theater_rect.visible

func on_reset_pressed():
	clean()
	setup()
