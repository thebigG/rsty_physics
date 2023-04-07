extends Main2D

#TODO: Add curves. Play with radius, x, y, etc
#Animates Godot in a circle using cos/x, sin/y and radius around a center_x,center_y 
#This draws a cos and sin wave.
#At the moment SineWave is just a Line2D node.
#This will probably stay like this until https://github.com/godot-rust/gdextension/issues/55
#is resolved. Then all this code will be moved into
#a rust gdextension; that it is  modular and scalable.
var sin_wave = SineWave2D.new()
var cos_wave = Line2D.new()

var y_axis = Line2D.new()
var x_axis = Line2D.new()	

#Add Control UI for origin and center_x, center_y
var origin = Vector2(400,200);

var obj = Sprite2D.new()
var obj_shape = Line2D.new()

var current_sinusoidal_output_val = 0
var current_cos_output_val = 0
var x_speed: float = 0.05  #radians/spped/rate
var y_speed: float = 0.10  #radians/spped/rate
var current_sin_input_val = 0
var current_cos_input_val = 0
var output_scale = 50

var sin_output_offset = origin.y  # Added to sin_output
var sin_label = Label.new()
var cos_label = Label.new()


var center_x = origin.x
var center_y = origin.y

var x_speed_spinner = SpinBox.new()
var y_speed_spinner = SpinBox.new()
var x_radius_spinner = SpinBox.new()
var y_radius_spinner = SpinBox.new()


var x_speed_label = Label.new()
var y_speed_label = Label.new()
var x_radius_label = Label.new()
var y_radius_label = Label.new()

#Add these fields as Control nodes
var x_radius = 100
var y_radius = 150

var position_delta = Vector2()

var spinner_x_offset = 200

var code_link = RichTextLabel.new()

# Called when the node e6nters the scene tree for the first time.
func _ready():
	#3d nodes don't play as nice when reloading the scene
	#So we only allow the nodes that are part of the "reloadable"
	#group to be reloaded.
	self.add_to_group("reloadable")
	var image = Image.load_from_file("res://assets/icon.svg")
	var texture = ImageTexture.create_from_image(image)
	
	obj.texture = texture
	obj.position = origin
	obj_shape.position = origin
	obj.scale = Vector2(0.3,0.3)
	
	y_axis.default_color = Color(Color.YELLOW)
	y_axis.default_color.a = 0.25
	
	y_axis.points = [Vector2(origin.x, origin.y-100), Vector2(origin.x, origin.y+300)]
	
	x_axis.default_color = Color(Color.YELLOW)
	x_axis.default_color.a = 0.25
	
	x_axis.points = [Vector2(origin.x-100, origin.y), Vector2(origin.x+300, origin.y)]
	
	sin_wave.default_color = Color.PURPLE
	super.new_game()
	sin_wave.points = super.get_sin_full_circle_2dvectors(30, 50, 2)
	sin_wave.position = origin

	cos_wave.points = get_cos_full_circle_2dvectors(30, 50, 2)
	cos_wave.position = origin
	
	sin_label.text = "Sin"
	cos_label.text = "Cos"
	cos_label.position.y += 25
	sin_label.add_theme_color_override("font_color", sin_wave.default_color)
	
	x_speed_spinner.position.y += 50
	x_speed_spinner.step = .01
	
	y_speed_spinner.position.y += x_speed_spinner.position.y + 30
	y_speed_spinner.step = .01
	
	x_speed_label.text = "X Speed/Angle(Radians):"
	y_speed_label.text = "Y Speed/Angle(Radians):"
	
	x_speed_label.position.y = x_speed_spinner.position.y
	y_speed_label.position.y = y_speed_spinner.position.y
	
	x_speed_spinner.position.x += spinner_x_offset
	y_speed_spinner.position.x += spinner_x_offset
	
	x_radius_spinner.max_value = x_radius * 106
	x_radius_spinner.value = x_radius 
	x_radius_spinner.value_changed.connect(update_x_radius)
	x_radius_spinner.position.y = y_speed_spinner.position.y + 30
	x_radius_spinner.position.x += spinner_x_offset
	
	y_radius_spinner.max_value = y_radius * 10
	y_radius_spinner.value = y_radius
	y_radius_spinner.value_changed.connect(update_y_radius)
	y_radius_spinner.position.y = x_radius_spinner.position.y + 30
	y_radius_spinner.position.x += spinner_x_offset
	
	x_radius_label.text = "X Radius:"
	
	x_radius_label.position.y = x_radius_spinner.position.y
	
	y_radius_label.text = "Y Radius:"
	
	y_radius_label.position.y = y_radius_spinner.position.y
	
	x_speed_spinner.value_changed.connect(update_cos_step)	
	x_speed_spinner.value = x_speed
	
	y_speed_spinner.value_changed.connect(update_sin_step)	
	y_speed_spinner.value = y_speed
	
	obj_shape.points = get_full_circle_shape_2dvectors_1phase(x_radius, y_radius)
	obj_shape.default_color = Color(Color.YELLOW)
	obj_shape.default_color.a = 0.25
	
	code_link.bbcode_enabled = true

	code_link.text = "[url=https://github.com/thebigG/rsty_physics/blob/main/godot/scenes/sin_cos_curve_lissajous_curve.gd][/https://github.com/thebigG/rsty_physics/blob/main/godot/scenes/sin_cos_curve_lissajous_curve.gd]"
	code_link.meta_underlined = true
	code_link.size = Vector2(600,100)
	code_link.position = Vector2(origin)
	code_link.position.y += 400
	code_link.size_flags_horizontal = 0 
	code_link.visible = true 
	code_link.meta_clicked.connect(open_browser_link)
	
	add_child(sin_wave)
	add_child(cos_wave)
	add_child(y_axis)
	add_child(x_axis)
	add_child(obj)
	add_child(obj_shape)
	add_child(sin_label)
	add_child(cos_label)
	add_child(x_speed_spinner)
	add_child(x_speed_label)
	add_child(y_speed_spinner)
	add_child(y_speed_label)
	add_child(x_radius_spinner)
	add_child(x_radius_label)
	add_child(y_radius_spinner)
	add_child(y_radius_label)
	add_child(code_link)

func open_browser_link(url: String):
	print(url)
	OS.shell_open(url)

func get_full_circle_shape_2dvectors_1phase(x_radius: float, y_radius: float) -> Array:
#	TODO:I think it's a matter of getting the all the points while the radius has not been covered?? Or going from o to 360n degrees.
	var points = []
	var i = 0
	var current_sin_input_val = 0
	var current_cos_input_val = 0
#	Possible implementation...not quite right 
	while i < 1 * (2 * PI):
#		Need to come up with all values for sin
#		Need to come up with all values for cos 
		var temp_current_sinusoidal_output_val = y_radius * sin(current_sin_input_val) 
		var temp_current_cos_output_val = x_radius * cos(current_cos_input_val) 
		points.append(Vector2(temp_current_cos_output_val, temp_current_sinusoidal_output_val))
		current_sin_input_val += y_speed
		current_cos_input_val += x_speed
		if y_speed < x_speed:
			i += y_speed
		else:
			i += x_speed
	return points


#In this case our curve is just a simple "circle". No fancy curves yet.
func calc_curve(speed_agle: float):
	pass

func update_sin_step(value: float):
	x_speed = value
	obj_shape.points = get_full_circle_shape_2dvectors_1phase(x_radius, y_radius)

func update_cos_step(value: float):
	y_speed = value
	obj_shape.points = get_full_circle_shape_2dvectors_1phase(x_radius, y_radius)
	
func update_x_radius(value: float):
	x_radius = value
	obj_shape.points = get_full_circle_shape_2dvectors_1phase(x_radius, y_radius)

func update_y_radius(value: float):
	y_radius = value
	obj_shape.points = get_full_circle_shape_2dvectors_1phase(x_radius, y_radius)

# TODO:Draw This Ellipses
func _physics_process(delta):
	#In this case our curve is just a simple "ellipses". No fancy curves yet.
	current_sinusoidal_output_val = sin(current_sin_input_val) 
	current_cos_output_val = cos(current_cos_input_val) 
	var old_position = Vector2(obj.position.x, obj.position.y)
	obj.position.x = center_x 
	obj.position.y = center_y
	obj.position.y += (y_radius * current_sinusoidal_output_val)  
	obj.position.x += (x_radius * current_cos_output_val)  
	position_delta.x = obj.position.x - old_position.x
	position_delta.y = obj.position.y - old_position.y
	
#	print("pos:" + str(obj.position -  origin))
	
	current_sin_input_val += y_speed
	current_cos_input_val += x_speed
	
#	print("Current input:" + str(current_cos_input_val/2*PI))

func _process(delta):
	pass
