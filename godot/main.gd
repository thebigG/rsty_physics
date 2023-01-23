extends Node3D

var views: OptionButton = OptionButton.new()

# Called when the node enters the scene tree for the first time.
func _ready():
	views.position.x += self.get_viewport().size.y/2
	for child in self.get_children():
		views.add_item(child.name)
		if child.name != "SinCos":
			child.set("visible", false)
	
	views.item_selected.connect(select_view)
	add_child(views)

func select_view(selection: int):
	print(views.get_item_text(selection))
	for child in get_children():
		if child.name != views.name:
			child.set("visible", false)
			if child.name ==  views.get_item_text(selection):
				child.set("visible", true)	

func _process(delta):
	pass
