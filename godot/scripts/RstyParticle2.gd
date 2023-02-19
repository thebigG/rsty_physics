class_name RstyParticle2

var position: RstyVector2
var velocity: RstyVector2
var gravity: RstyVector2

var mass: float

func _init(position: RstyVector2, speed: float, direction: float, grav: float = 0) -> void:
	self.position = RstyVector2.new(position.get_x(), position.get_y())
	self.velocity = RstyVector2.new(0, 0)
	self.velocity.set_length(speed)
	self.velocity.set_angle(direction)
	self.gravity = RstyVector2.new(0, grav)

func accelerate(accel: RstyVector2) -> void:
	self.velocity.add_to(accel)

func update() -> void:
	self.velocity.add_to(self.gravity)
	self.position.add_to(self.velocity)


func angle_to(p2: RstyParticle2) -> float:
	return atan2(p2.position.y -  self.position.y, p2.position.x -  self.position.x)
	

func distance_to(p2: RstyParticle2) -> float:
	var dx = p2.position.x - self.position.x
	var dy = p2.position.y - self.position.y
	
	return sqrt(pow(dx, 2) + pow(dy, 2))

func gravitate_to(p2: RstyParticle2):
	var grav = RstyVector2.new(0,0)
	var dist = self.distance_to(p2)
	
	grav.set_length(p2.mass / pow(dist, 2))
	grav.set_angle(self.angle_to(p2))
	
	self.velocity.add_to(grav)
