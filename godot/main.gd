extends Main

var sin_wave = SineWave2D.new();
#var sin_wav = SineWave2D.new();

# Called when the node enters the scene tree for the first time.
func _ready():
	sin_wave.default_color = Color.PURPLE;
	super.new_game()
	var points = get_sin_full_circle_2dvectors(30, 50)
	for p in points:
		sin_wave.points.append(p)	
	sin_wave.points = get_sin_full_circle_2dvectors(30, 50)
	sin_wave.position = Vector2(100,200)
	
#	sin_wave.draw_wave()
	
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
