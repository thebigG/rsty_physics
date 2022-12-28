extends Main

#This draws a cos and sin wave.
#At the moment SineWave is just a Line2D node.
#This will probablyustay like this until https://github.com/godot-rust/gdextension/issues/55
#is resolved. Then all this code will be moved into
# a rust gdextension; that it is  modular and scalable.
var sin_wave = SineWave2D.new();
var cos_wave = Line2D.new();

var y_axis = Line2D.new();
var x_axis = Line2D.new();

var Origin = Vector2(100,200);
#var sin_wav = SineWave2D.new();

# Called when the node enters the scene tree for the first time.
func _ready():
	y_axis.default_color = Color(Color.YELLOW)
	y_axis.default_color.a = 0.25
	
	y_axis.points = [Vector2(Origin.x, Origin.y-100), Vector2(Origin.x, Origin.y+300)]
	
	
	x_axis.default_color = Color(Color.YELLOW)
	x_axis.default_color.a = 0.25
	
	x_axis.points = [Vector2(Origin.x-100, Origin.y), Vector2(Origin.x+300, Origin.y)]
	
	sin_wave.default_color = Color.PURPLE
	super.new_game()
	sin_wave.points = get_sin_full_circle_2dvectors(30, 50, 4)
	sin_wave.position = Origin

	cos_wave.points = get_cos_full_circle_2dvectors(30, 50, 4)
	cos_wave.position = Origin
	
	add_child(sin_wave)
	add_child(cos_wave)
	add_child(y_axis)
	add_child(x_axis)

func get_sin_full_circle_2dvectors(degrees_delta: int, scale: int, number_of_phases: int) -> Array:
	var points = []
	var i = 0;
	while i < number_of_phases * PI:
		var x = sin(i)
		points.append(Vector2(i*scale, x*scale))
		i += deg_to_rad(degrees_delta)
	return points

func get_cos_full_circle_2dvectors(degrees_delta: int, scale: int, number_of_phases: int ) -> Array:
	var points = []
	var i = 0;
	while i < number_of_phases * PI:
		var x = cos(i)
		points.append(Vector2(i*scale, x*scale))
		i += deg_to_rad(degrees_delta)
	return points

func _process(delta):
	pass
