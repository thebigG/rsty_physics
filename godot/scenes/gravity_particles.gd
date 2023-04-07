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

var particle1_sprite = Sprite2D.new()
var particle1 =  RstyParticle2.new(RstyVector2.new(), 10, -PI/2)

var particle2_sprite = Sprite2D.new()
var particle2 =  RstyParticle2.new()


var particle_mass = 15000

var current_sinusoidal_output_val = 0
var current_cos_output_val = 0
var current_sin_input_val = 0
var output_scale = 50

var sin_output_offset = Origin.y  # Added to sin_output
var sin_label = Label.new()
var cos_label = Label.new()

var center_x = Origin.x
var center_y = Origin.y

var mass_spinner = SpinBox.new()
var mass_spinner_label = Label.new()

var radius = 100

var code_link = RichTextLabel.new()

var v: RstyVector2 = RstyVector2.new(0, 0)

var y_velocity: float = 0
var x_velocity: float = 2
var x_acceleration = 0.001

var apply_x_acceleration_button  = Button.new()

var apply_x_accel: bool  = false


# Called when the node enters the scene tree for the first time.
func _ready():
	#3d nodes don't play as nice when reloading the scene
	#So we only allow the nodes that are part of the "reloadable"
	#group to be reloaded.
	self.add_to_group("reloadable")
	var image = Image.load_from_file("res://assets/icon.svg")
	var texture = ImageTexture.create_from_image(image)
	
	var sun_image = Image.load_from_file("res://assets/sun.png")
	var sun_texture = ImageTexture.create_from_image(sun_image)
	
	particle1_sprite.texture = texture
	particle1.position.x = Origin.x + 200
	particle1.position.y = Origin.y
	particle1_sprite.position = particle1.position.get_godot_vector()
	particle1_sprite.scale = Vector2(0.3,0.3)
#	particle1_sprite.modulate = Color(1, 0, 0, 0.5)	
	
	particle2_sprite.texture = sun_texture
	particle2.position.x = Origin.x
	particle2.position.y = Origin.y
	particle2_sprite.position = particle2.position.get_godot_vector()
	particle2_sprite.scale = Vector2(0.030 * particle_mass/pow(10,4),0.030 * particle_mass/pow(10,4))
#	particle2_sprite.modulate = Color(1, 0, 0, 0.5)
	particle2.mass = particle_mass

#	Have to do this since godot has no "set_length" for vectors. https://github.com/godotengine/godot/pull/37704

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

	mass_spinner.position.y += 50
	mass_spinner.step = 100
	mass_spinner.min_value = 0
	mass_spinner.max_value = abs(particle_mass * 100) 

	mass_spinner_label.text = "Sun Mass:"

	mass_spinner_label.position.y = mass_spinner.position.y

	mass_spinner.position.x += 75

	mass_spinner.value_changed.connect(update_mass)
	mass_spinner.value = particle_mass

	code_link.bbcode_enabled = true

	code_link.text = "[url=https://github.com/thebigG/rsty_physics/blob/main/godot/scenes/gravity_particles.gd][/https://github.com/thebigG/rsty_physics/blob/main/godot/scenes/gravity_particles.gd]"
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
	add_child(particle1_sprite)
	add_child(particle2_sprite)
	add_child(sin_label)
	add_child(cos_label)
	add_child(mass_spinner)
	add_child(mass_spinner_label)
	
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
	if(particle_mass != particle2.mass):
		particle2.mass = particle_mass
		particle2_sprite.scale = Vector2(0.030 * particle_mass/pow(10,4),0.030 * particle_mass/pow(10,4))
	particle1.gravitate_to(particle2)
	particle1.update()
	
	particle1_sprite.position = particle1.position.get_godot_vector() 
	

func open_browser_link(url: String):
	print(url)
	OS.shell_open(url)

func update_apply_x_accel():
	apply_x_accel = !apply_x_accel
#In this case our curve is just a simple "circle". No fancy curves yet.
func calc_curve(speed_agle: float):
	pass

func update_mass(value: float):
	particle_mass = value

func update_x_acceleration(value: float):
	x_acceleration = value

func _physics_process(delta):
	update_particle()
	

func from_particle_gravitate_to(particle: Vector2, v: Vector2):
	var grav  = Vector2(0,0)
	var distance = v.distance_to(v)
#	grav = get_vector2_with_length()

func _process(delta):
	pass
