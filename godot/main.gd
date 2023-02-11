extends Node3D

var views: OptionButton = OptionButton.new()
var reset: Button = Button.new()
var current_node: Node
var sin_cos_scene = preload("res://sin_cos.tscn")
var velocity_particle_scene = preload("res://velocity_particle.tscn")
var sin_cos_curve_ellips_anim_scene = preload("res://sin_cos_curve_ellipses_anim.tscn")
var arc_tan_scene = preload("res://arc_tan.tscn")
#var sin_cos_curve_ellips_anim = preload("res://sin_cos_curve_ellipses_anim.gd")

# Called when the node enters the scene tree for the first time.
func _ready():
	views.position.x += self.get_viewport().size.y/2
	reset.text = "Reset"
	reset.name = "reset"
	var sin_cos_node: Main2D = sin_cos_scene.instantiate()
	current_node = sin_cos_node
	var velocity_particle_node: Main2D = velocity_particle_scene.instantiate()
	add_child(sin_cos_node)
	add_child(velocity_particle_node)
	add_child(reset)
	views.add_item(sin_cos_node.name)
	views.add_item(velocity_particle_node.name)
#	for child in self.get_children():
#		views.add_item(child.name)
#		if child.name != "SinCos":
#			child.set("visible", false)
#		else:
#			print("scene-->" + sin_cos_node.get_tree().current_scene.name)
	views.item_selected.connect(select_view)
	reset.pressed.connect(reset_node)
	
	add_child(views)

func reset_node():
	current_node.queue_free()
#	TODO:Needs to be updated to get current scene of current scene. A Map maybe?
	current_node = sin_cos_scene.instantiate()
	add_child(current_node)
	views.item_selected.emit()
#	update_views(views.selected)

func select_view(selection: int):
	update_views(selection)


func update_views(selection: int):
	for child in get_children():
		if child.name != views.name:
			child.set("visible", false)
			if child.name == views.get_item_text(selection):
				current_node = child
			if child.name == "reset":
				child.set("visible", true)

func _process(delta):
	pass
