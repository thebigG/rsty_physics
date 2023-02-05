extends Main2D

#TODO: Add curves. Play with radius, x, y, etc
#Animates Godot in a circle using cos/x, sin/y and radius around a center_x,center_y 
#This draws a cos and sin wave.
#At the moment SineWave is just a Line2D node.
#This will probably stay like this until https://github.com/godot-rust/gdextension/issues/55
#is resolved. Then all this code will be moved into
#a rust gdextension; that it is  modular and scalable.
var sin_wave = SineWave2D.new();
var cos_wave = Line2D.new();

var y_axis = Line2D.new();
var x_axis = Line2D.new();	

var Origin = Vector2(300,200);

var particle = Sprite2D.new()

var particle_position = Vector2()

var particle_velocity = Vector2()

var current_sinusoidal_output_val = 0
var current_cos_output_val = 0
var speed = 0.05  #radians/spped/rate
var current_sin_input_val = 0
var output_scale = 50

var sin_output_offset = Origin.y  # Added to sin_output
var sin_label = Label.new()
var cos_label = Label.new()

var center_x = Origin.x
var center_y = Origin.y

var speed_spinner = SpinBox.new()
var speed_label = Label.new()

var radius = 100

var code_link = RichTextLabel.new()

# Called when the node enters the scene tree for the first time.
func _ready():
	Origin.length()
	var image = Image.load_from_file("res://icon.svg")
	var texture = ImageTexture.create_from_image(image)
	
	particle.texture = texture
	particle.position = particle_position
	particle.scale = Vector2(0.3,0.3)
	
#	Have to do this since godot has no "set_length" for vectors. https://github.com/godotengine/godot/pull/37704
	particle_velocity = get_vector2_with_length(particle_velocity, speed)
	
	y_axis.default_color = Color(Color.YELLOW)
	y_axis.default_color.a = 0.25
	
	y_axis.points = [Vector2(Origin.x, Origin.y-100), Vector2(Origin.x, Origin.y+300)]
	
	x_axis.default_color = Color(Color.YELLOW)
	x_axis.default_color.a = 0.25
	
	x_axis.points = [Vector2(Origin.x-100, Origin.y), Vector2(Origin.x+300, Origin.y)]
	
	sin_wave.default_color = Color.PURPLE
	super.new_game()
	sin_wave.points = get_sin_full_circle_2dvectors(30, 50, 2)
	sin_wave.position = Origin

	cos_wave.points = get_cos_full_circle_2dvectors(30, 50, 2)
	cos_wave.position = Origin
	
	sin_label.text = "Sin"
	cos_label.text = "Cos"
	cos_label.position.y += 25
	sin_label.add_theme_color_override("font_color", sin_wave.default_color)
	
	speed_spinner.position.y += 50
	speed_spinner.step = .01
	
	speed_label.text = "Speed/Angle(Radians):"
	
	speed_label.position.y = speed_spinner.position.y
	
	speed_spinner.position.x += 175
	
	speed_spinner.value_changed.connect(update_sin_step)	
	speed_spinner.value = speed
	
	code_link.bbcode_enabled = true

	code_link.text = "[url=https://github.com/thebigG/rsty_physics/blob/main/godot/velocity_particle.gd][/https://github.com/thebigG/rsty_physics/blob/main/godot/velocity_particle.gd]"
	code_link.meta_underlined = true
	code_link.size = Vector2(600,100)
	code_link.position = Vector2(Origin)
	code_link.position.y += 400
	code_link.size_flags_horizontal = 0 
	code_link.visible = true 
	code_link.meta_clicked.connect(open_browser_link)
	
	add_child(sin_wave)
	add_child(cos_wave)
	add_child(y_axis)
	add_child(x_axis)
	add_child(particle)
	add_child(sin_label)
	add_child(cos_label)
	add_child(speed_spinner)
	add_child(speed_label)
	add_child(code_link)

func get_vector2_with_length(v: Vector2, length) -> Vector2 :
	return v.normalized() * length

func get_vector2_with_angle(v: Vector2, angle: float) -> Vector2:
	var new_vector = Vector2()
	var length = v.length()
	new_vector.x = cos(angle) * length
	new_vector.y = sin(angle) * length
	return  new_vector
  

func update_particle():
#	This is kind of convoluted
	current_sinusoidal_output_val = sin(current_sin_input_val) 
	current_cos_output_val = cos(current_sin_input_val)
	
	particle_position.x = center_x
	particle_position.y = center_y
	particle_velocity.y = (radius * current_sinusoidal_output_val)  
	particle_velocity.x = (radius * current_cos_output_val)  
	
	particle_position += particle_velocity
	
	particle.position = particle_position
	current_sin_input_val += speed
#func get_particle2d(position: Vector2) -> Vector2:
#	pass

func open_browser_link(url: String):
	print(url)
	OS.shell_open(url)

#In this case our curve is just a simple "circle". No fancy curves yet.
func calc_curve(speed_agle: float):
	pass

func update_sin_step(value: float):
	speed = value

func _physics_process(delta):
	#In this case our curve is just a simple "circle". No fancy curves yet.
	update_particle()
	
func _process(delta):
	pass
