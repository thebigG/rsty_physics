extends Main

var sin_wave = Line2D.new();

# Called when the node enters the scene tree for the first time.
func _ready():
	sin_wave.default_color = Color.RED;
	super.new_game()
	sin_wave.points = get_sin_full_circle_2dvectors(30, 50)
	sin_wave.position = Vector2(100,200)
	add_child(sin_wave)

func get_sin_full_circle_2dvectors(degrees_delta: int, scale) -> Array:
	var points = []
	var i = 0;
	while i < 2 * PI:
		var x = sin(i)
		points.append(Vector2(i*scale, x*scale))
		i += deg_to_rad(degrees_delta)
	return points

func _process(delta):
	pass
