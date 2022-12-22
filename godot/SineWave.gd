extends SineWave2D


# Called when the node enters the scene tree for the first time.
func _ready():
#	pass # Replace with function body.
	var list = Array()
	list.append(12)
	print(list)
	print(typeof(list))
	self.draw_wave()
#	var l = self.draw_wave()
#	l.append(Vector2(1,3))
#	print(l)
#	l.append(12)
#	print(l)
#	print(self.draw_wave())


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
