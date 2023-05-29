extends Node3D

var views: OptionButton = OptionButton.new()
var reset: Button = Button.new()
var current_node: Node
var scene_dict = {}
#var sin_cos_curve_ellips_anim = preload("res://sin_cos_curve_ellipses_anim.gd")

# Called when the node enters the scene tree for the first time.
func _ready():
	views.position.x += self.get_viewport().size.y/2
	reset.text = "Reset"
	reset.name = "reset"
	var scene_paths = dir_contents("res://scenes")
	print(scene_paths)
	current_node = init_scene(load(scene_paths[0]))
	add_child(current_node)
	for scene_path in scene_paths.slice(1, len(scene_paths)):
		add_child(init_scene(load(scene_path)))
	
	print(scene_dict)
	for node in scene_dict:
		views.add_item(node)
	
	add_child(reset)
	update_views(views.selected)
	views.item_selected.connect(select_view)
	reset.pressed.connect(reset_node)
	
	current_node.visible = true
	
	add_child(views)

func init_scene(scene):
	var new_node: Node = scene.instantiate()
	scene_dict[new_node.name] = scene
	return new_node

func reset_node():
	var current_node_name = current_node.name
	if not(current_node.is_in_group("reloadable")):
		return
	var new_scene = scene_dict[current_node_name]
	current_node.queue_free()
#	TODO:Needs to be updated to get current scene of current scene. A Map maybe?
	current_node = new_scene.instantiate()
#	Always get the name after add_child call since add_child adds symbols to the name of the node
	add_child(current_node)
	scene_dict[current_node.name] = new_scene
	views.set_item_text(views.selected, current_node.name)
	update_views(views.selected)

func select_view(selection: int):
	update_views(selection)


func update_views(selection: int):
	for child in get_children():
		if child.name != views.name:
#			child.set("visible", false)
			disable_and_hide_node(child)
			if child.name == views.get_item_text(selection):
				current_node = child
				print("visible:" + current_node.name)
#				current_node.set("visible", true)
				enable_and_show_node(child)
			if child.name == "reset":
#				child.set("visible", true)
				enable_and_show_node(child)

# I think this is a much better way of updating views as we switch between them
# I think this will also disable the physics of those nodes.
func disable_and_hide_node(node:Node) -> void:
	node.process_mode = Node.PROCESS_MODE_DISABLED  # = Mode: Disabled
	node.hide()

func enable_and_show_node(node:Node) -> void:
	node.process_mode = Node.PROCESS_MODE_INHERIT # = Mode: Inherit
	node.show()


func dir_contents(path: String):
	var dir = DirAccess.open(path)
	var files = []
	if dir:
		dir.list_dir_begin()
		var file_name = dir.get_next()
		while file_name != "":
			if dir.current_is_dir():
				print("Found directory: " + file_name)
			else:
				print("Found file: " + file_name)
				if file_name.split(".")[1] == "tscn":
					files.append(path + "/" + file_name)
				
			file_name = dir.get_next()
	else:
		print("An error occurred when trying to access the path.")
	return files

func _process(delta):
	pass
