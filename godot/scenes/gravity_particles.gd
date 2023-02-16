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


var y_axis_length = 600
var x_axis_length = 600

var y_axis = Line2D.new();
var x_axis = Line2D.new();

var Origin = Vector2(500,200);

var particle1 = Sprite2D.new()

#var particle_position = Origin
var particle_position = Vector2()

var particle_velocity = Vector2()

var current_sinusoidal_output_val = 0
var current_cos_output_val = 0
var current_sin_input_val = 0
var output_scale = 50

var sin_output_offset = Origin.y  # Added to sin_output
var sin_label = Label.new()
var cos_label = Label.new()

var center_x = Origin.x
var center_y = Origin.y

var x_velocity_spinner = SpinBox.new()
var velocity_label = Label.new()

var x_acceleration_spinner = SpinBox.new()
var x_acceleration_label = Label.new()

var radius = 100

var code_link = RichTextLabel.new()

var v: RstyVector2 = RstyVector2.new()

var y_velocity: float = 0
var x_velocity: float = 2
var x_acceleration = 0.001

var apply_x_acceleration_button  = Button.new()

var apply_x_accel: bool  = false


# Called when the node enters the scene tree for the first time.
func _ready():
	var image = Image.load_from_file("res://assets/icon.svg")
	var texture = ImageTexture.create_from_image(image)
	
	particle_position = Vector2(Origin)
	particle1.texture = texture
	particle1.position = particle_position
	particle1.scale = Vector2(0.3,0.3)
	particle1.modulate = Color(1, 0, 0, 0.5)

#	Have to do this since godot has no "set_length" for vectors. https://github.com/godotengine/godot/pull/37704
	particle_velocity = get_vector2_with_length(particle_velocity, x_velocity)
	particle_velocity = Vector2(2,2)

	y_axis.default_color = Color(Color.YELLOW)
	y_axis.default_color.a = 0.25
	
	y_axis.points = [Vector2(Origin.x, Origin.y-y_axis_length), Vector2(Origin.x, Origin.y+y_axis_length)]

	x_axis.default_color = Color(Color.YELLOW)
	x_axis.default_color.a = 0.25

	x_axis.points = [Vector2(Origin.x-x_axis_length, Origin.y), Vector2(Origin.x+x_axis_length, Origin.y)]

	sin_wave.default_color = Color.PURPLE

	sin_wave.points = get_sin_full_circle_2dvectors(30, 50, 2)
	sin_wave.position = Origin

	cos_wave.points = get_cos_full_circle_2dvectors(30, 50, 2)
	cos_wave.position = Origin

	sin_label.text = "Sin"
	cos_label.text = "Cos"
	cos_label.position.y += 25
	sin_label.add_theme_color_override("font_color", sin_wave.default_color)

	x_velocity_spinner.position.y += 50
	x_velocity_spinner.step = .01
	x_velocity_spinner.min_value = abs(x_velocity) * -1
	x_velocity_spinner.max_value = abs(x_velocity) 

	velocity_label.text = "x velocity:"

	velocity_label.position.y = x_velocity_spinner.position.y

	x_velocity_spinner.position.x += 75
	
	x_velocity_spinner.min_value = -1000.0
	x_velocity_spinner.max_value = 1000.0

	x_velocity_spinner.value_changed.connect(update_x_velocity)
	x_velocity_spinner.value = x_velocity
	
	x_acceleration_spinner.position.y += 100
	x_acceleration_spinner.step = 0.001
	x_acceleration_spinner.min_value = abs(x_acceleration) * -1
	x_acceleration_spinner.max_value = abs(x_acceleration) 

	x_acceleration_label.text = "x acceleration:"

	x_acceleration_label.position.y = x_acceleration_spinner.position.y

	x_acceleration_spinner.position.x += 125
	
	x_acceleration_spinner.min_value = -1000.0
	x_acceleration_spinner.max_value = 1000.0

	x_acceleration_spinner.value_changed.connect(update_x_acceleration)
	x_acceleration_spinner.value = x_acceleration
	
	apply_x_acceleration_button.text = "Press To Accelerate"
	apply_x_acceleration_button.position.y = x_acceleration_spinner.position.y
	apply_x_acceleration_button.position.x = x_acceleration_spinner.position.x + 100
	apply_x_acceleration_button.button_down.connect(update_apply_x_accel)
	apply_x_acceleration_button.button_up.connect(update_apply_x_accel)

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
	add_child(particle1)
	add_child(sin_label)
	add_child(cos_label)
	add_child(x_velocity_spinner)
	add_child(velocity_label)
	
	
	add_child(x_acceleration_spinner)
	add_child(x_acceleration_label)
	add_child(apply_x_acceleration_button)
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
	if(particle_position.x > Origin.x + x_axis_length or particle_position.x < Origin.x - x_axis_length):
		x_velocity  = -1 * x_velocity
		x_acceleration = -1 * x_acceleration
	
	x_acceleration_spinner.value = x_acceleration
	
	if(apply_x_accel):
		x_velocity += x_acceleration
	
	x_velocity_spinner.value = x_velocity	
	particle_velocity.y = y_velocity
	particle_velocity.x = x_velocity
	
	particle_position += particle_velocity
	
	particle1.position = particle_position
	

func open_browser_link(url: String):
	print(url)
	OS.shell_open(url)

func update_apply_x_accel():
	apply_x_accel = !apply_x_accel
#In this case our curve is just a simple "circle". No fancy curves yet.
func calc_curve(speed_agle: float):
	pass

func update_x_velocity(value: float):
	x_velocity = value

func update_x_acceleration(value: float):
	x_acceleration = value

func _physics_process(delta):
	#In this case our curve is just a simple "circle". No fancy curves yet.
	update_particle()
	

func gravitate_to(v: Vector2):
	var grav  = Vector2(0,0)
	var distance = v.distance_to(v)
#	grav = get_vector2_with_length()

func _process(delta):
	pass
