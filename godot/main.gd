extends Node3D

var views: OptionButton = OptionButton.new()
var reset: Button = Button.new()
var current_node = "SinCos"

# Called when the node enters the scene tree for the first time.
func _ready():
	views.position.x += self.get_viewport().size.y/2
	reset.text = "Reset"
	for child in self.get_children():
		views.add_item(child.name)
		if child.name != "SinCos":
			child.set("visible", false)
#			print("visible:"+ child.name)
	views.item_selected.connect(select_view)
	
	reset.pressed.connect(reset_node)
	
	add_child(views)
#	add_child(reset)

func reset_node():
#	print(current_node)
	get_node(current_node).get_tree().reload_current_scene()

func select_view(selection: int):
#	print(views.get_item_text(selection))
	for child in get_children():
		if child.name != views.name:
			child.set("visible", false)
			print("visible:"+ child.name)
			if child.name ==  views.get_item_text(selection):
				current_node = child.name
				child.set("visible", true)	

func _process(delta):
	pass
