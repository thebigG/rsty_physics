extends Main

var sin_wave = Line2D.new();

# Called when the node enters the scene tree for the first time.
func _ready():
	sin_wave.default_color = Color.RED;
	super.new_game()
	sin_wave.points = get_sin_full_circle_2dvectors(30, 100)
	sin_wave.position = Vector2(100,200)
	add_child(sin_wave)

func get_sin_full_circle_2dvectors(degrees_delta: int, scale) -> Array:
	var points = []
	for point in range(0, 360+degrees_delta, degrees_delta):
		var x = sin(deg_to_rad(point));
		points.append(Vector2(point, x*scale))
	return points

func _process(delta):
	pass
