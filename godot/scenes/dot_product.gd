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

var red_vector_shape = Line2D.new()
var blue_vector_shape = Line2D.new()

var current_sinusoidal_output_val = 0
var current_cos_output_val = 0
var current_sin_input_val = 0
var current_cos_input_val = 0
var output_scale = 50

var sin_output_offset = origin.y  # Added to sin_output
var sin_label = CheckButton.new()
var cos_label = CheckButton.new()

var center_x = origin.x
var center_y = origin.y

var blue_y2_spinner = SpinBox.new()
var blue_x2_spinner = SpinBox.new()
var blue_y2_label = Label.new()
var blue_x2_label = Label.new()

var red_x2_spinner = SpinBox.new()
var red_y2_spinner = SpinBox.new()
var red_x2_label = Label.new()
var red_y2_label = Label.new()

var blue_y2 = 50  #radians/spped/rate
var blue_x2 = 50  #radians/spped/rate
#Add these fields as Control nodes
var red_x2 = 100
var red_y2 = 35

var position_delta = Vector2()
var spinner_x_offset = 100

var vector_step = 5

var dot_product_label = Label.new()
var dot_product_value = dot_product(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))

var dot_product_normalized_label = Label.new()
var dot_product_normalized_value = dot_product_normalized(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))

var angle_between_label = Label.new()
var angle_between_value = angle_between(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))

var code_link = RichTextLabel.new()

var particle: Particle2D = Particle2D.new()

# Called when the node e6nters the scene tree for the first time.
func _ready():
#	OS.shell_open("google.com")
	var image = Image.load_from_file("res://assets/icon.svg")
	var texture = ImageTexture.create_from_image(image)

	red_vector_shape.position = origin
	blue_vector_shape.position = origin
	
	y_axis.default_color = Color(Color.YELLOW)
	y_axis.default_color.a = 0.25
	
	y_axis.points = [Vector2(origin.x, origin.y-100), Vector2(origin.x, origin.y+300)]
	
	x_axis.default_color = Color(Color.YELLOW)
	x_axis.default_color.a = 0.25
	
	x_axis.points = [Vector2(origin.x-100, origin.y), Vector2(origin.x+300, origin.y)]

	
	sin_wave.default_color = Color.PURPLE
	super.new_game()
	sin_wave.points = get_sin_full_circle_2dvectors(30, 50, 2)
	sin_wave.position = origin
	sin_wave.visible = false
	cos_wave.visible = false

	cos_wave.points = get_cos_full_circle_2dvectors(30, 50, 2)
	cos_wave.position = origin
	
	sin_label.text = "Sin"
	cos_label.text = "Cos"
	cos_label.position.y += 25
	sin_label.add_theme_color_override("font_color", sin_wave.default_color)
	
	blue_y2_spinner.position.y += 50
	blue_y2_spinner.step = vector_step
	
	blue_x2_spinner.position.y += blue_y2_spinner.position.y + 30
	blue_x2_spinner.step = vector_step
	
	blue_y2_label.text = "Blue Y2:"
	blue_y2_label.add_theme_color_override("font_color", Color.BLUE)
	blue_x2_label.add_theme_color_override("font_color", Color.BLUE)
	blue_x2_label.text = "Blue X2"
	
	blue_y2_label.position.y = blue_y2_spinner.position.y
	blue_x2_label.position.y = blue_x2_spinner.position.y
	
	blue_y2_spinner.position.x += spinner_x_offset
	blue_x2_spinner.position.x += spinner_x_offset
	
	red_x2_spinner.max_value = red_x2 * 10
	red_x2_spinner.min_value = red_x2_spinner.max_value * -1
	red_x2_spinner.value = red_x2 
	red_x2_spinner.value_changed.connect(update_red_x2)
	red_x2_spinner.position.y = blue_x2_spinner.position.y + 30
	red_x2_spinner.position.x += spinner_x_offset
	
	red_y2_spinner.max_value = red_y2 * 10
	red_y2_spinner.min_value = red_y2_spinner.max_value * -1
	red_y2_spinner.value = red_y2
	red_y2_spinner.value_changed.connect(update_red_y2)
	red_y2_spinner.position.y = red_x2_spinner.position.y + 30
	red_y2_spinner.position.x += spinner_x_offset
	
	red_x2_label.text = "Red X2:"
	red_x2_label.add_theme_color_override("font_color", Color.RED)
	
	red_x2_label.position.y = red_x2_spinner.position.y
	
	red_y2_label.text = "Red Y2:"
	red_y2_label.add_theme_color_override("font_color", Color.RED)
	
	red_y2_label.position.y = red_y2_spinner.position.y
	
	blue_y2_spinner.value_changed.connect(update_blue_y2)	
	blue_y2_spinner.max_value = 100 * blue_y2
	blue_y2_spinner.min_value =  blue_y2_spinner.max_value * -1
	blue_y2_spinner.value = blue_y2
	
	blue_x2_spinner.value_changed.connect(update_blue_x2)	
	blue_x2_spinner.max_value = 100 * blue_x2
	blue_x2_spinner.min_value =  blue_x2_spinner.max_value * -1
	blue_x2_spinner.value = blue_x2
	
	red_vector_shape.points = get_vector_from_origin_2dvectors(red_x2, red_y2)
	red_vector_shape.default_color = Color.RED
	red_vector_shape.default_color.a = 0.25
	
	blue_vector_shape.points = get_vector_from_origin_2dvectors(blue_x2, blue_y2)
	blue_vector_shape.default_color = Color.BLUE
	blue_vector_shape.default_color.a = 0.25
	
	dot_product_label.text = "Dot Product:" + str(dot_product_value)
	dot_product_label.add_theme_color_override("font_color", Color.WHITE)
	dot_product_label.position.y = red_y2_spinner.position.y  + 30
	
	dot_product_normalized_label.text = "Dot Product Normalized:" + str(dot_product_normalized_value)
	dot_product_normalized_label.add_theme_color_override("font_color", Color.WHITE)
	dot_product_normalized_label.position.y = dot_product_label.position.y  + 30
	
	
	angle_between_label.text = "Angle Between Vectors:" + str(angle_between_value)
	angle_between_label.add_theme_color_override("font_color", Color.WHITE)
	angle_between_label.position.y = dot_product_normalized_label.position.y  + 30
	
	code_link.bbcode_enabled = true

	code_link.text = "[url=https://github.com/thebigG/rsty_physics/blob/main/godot/dot_product.gd][/https://github.com/thebigG/rsty_physics/blob/main/godot/dot_product.gd]"
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
	add_child(red_vector_shape)
	add_child(blue_vector_shape)
	add_child(sin_label)
	add_child(cos_label)
	add_child(blue_y2_spinner)
	add_child(blue_y2_label)
	add_child(blue_x2_spinner)
	add_child(blue_x2_label)
	add_child(red_x2_spinner)
	add_child(red_x2_label)
	add_child(red_y2_spinner)
	add_child(red_y2_label)
	add_child(dot_product_label)
	add_child(dot_product_normalized_label)
	add_child(code_link)
	add_child(angle_between_label)


func open_browser_link(url: String):
	print(url)
	OS.shell_open(url)

func get_cos_full_circle_2dvectors(degrees_delta: int, scale: int, number_of_phases: int ) -> Array:
	var points = []
	var i = 0;
	while i < number_of_phases * (2 * PI):
		var y = cos(i)
		points.append(Vector2(i*scale, y*scale))
		i += deg_to_rad(degrees_delta)
	return points


func _input(event):
   # Mouse in viewport coordinates.
	if event is InputEventMouseMotion:
#		print(event.position)
		var dy  = center_y - event.position.y
		var dx  = center_x - event.position.x

func get_vector_from_origin_2dvectors(x: float, y: float) -> Array:
#	TODO:I think it's a matter of getting the all the points while the radius has not been covered?? Or going from o to 360n degrees.
	var points = []
	points.append(Vector2(0,0))
	points.append(Vector2(x, y))
	return points

#In this case our curve is just a simple "circle". No fancy curves yet.
func calc_curve(speed_agle: float):
	pass

func update_red_x2(value: float):
	red_x2 = value
	red_vector_shape.points = get_vector_from_origin_2dvectors(red_x2, red_y2)
	dot_product_value = dot_product(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))
	dot_product_normalized_value = dot_product_normalized(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))
	angle_between_value = angle_between(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))

func update_red_y2(value: float):
	red_y2 = value
	red_vector_shape.points = get_vector_from_origin_2dvectors(red_x2, red_y2)
	dot_product_value = dot_product(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))
	dot_product_normalized_value = dot_product_normalized(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))
	angle_between_value = angle_between(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))

func update_blue_x2(value: float):
	blue_x2 = value
	blue_vector_shape.points = get_vector_from_origin_2dvectors(blue_x2, blue_y2)
	dot_product_value = dot_product(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))
	dot_product_normalized_value = dot_product_normalized(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))
	angle_between_value = angle_between(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))

func update_blue_y2(value: float):
	blue_y2 = value
	blue_vector_shape.points = get_vector_from_origin_2dvectors(blue_x2, blue_y2)
	dot_product_value = dot_product(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))
	dot_product_normalized_value = dot_product_normalized(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))
	angle_between_value = angle_between(Vector2(red_x2, red_y2), Vector2(blue_x2, blue_y2))


# TODO:Draw This Ellipses
func _physics_process(delta):
	#In this case our curve is just a simple "ellipses". No fancy curves yet.
	current_sinusoidal_output_val = sin(current_sin_input_val) 
	current_cos_output_val = cos(current_cos_input_val) 

	current_sin_input_val += blue_x2
	current_cos_input_val += blue_y2

	dot_product_label.text = "Dot Product:" + str(dot_product_value)
	dot_product_normalized_label.text = "Dot Product Normalized:" + str(dot_product_normalized_value)	
	dot_product_normalized_label.text = "Dot Product Normalized:" + str(dot_product_normalized_value)
	angle_between_label.text = "Angle Between Vectors:" + str(snapped(rad_to_deg(angle_between_value), 0.001))
#	print(angle_between_value)6
	pass


func dot_product(a: Vector2, b: Vector2):
	return ((a.x * b.x) + (a.y * b.y))

func magnitude(v: Vector2):
	return sqrt(pow(v.x, 2) + pow(v.y, 2))

func normalize(v: Vector2):
	return Vector2(v.x/magnitude(v), v.y/magnitude(v) )

func dot_product_normalized(a: Vector2, b: Vector2):
	return snapped(dot_product(normalize(a),normalize(b)), 0.001 )  

func angle_between(a: Vector2, b: Vector2):
	#	dp = mag(a) * mag(b) *  cos(a)
	#	acos(dp/mag(a)/mag(b)) = a
	var dp = dot_product(a,b)
	var mag_a = magnitude(a)
	var mag_b = magnitude(b)
	var a_result = acos(dp/mag_a/mag_b)
	return a_result

func _process(delta):
	pass
