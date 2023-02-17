class_name RstyParticle2

var position: RstyVector2
var velocity: RstyVector2
var gravity: RstyVector2

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

#	func to_string() -> String:
#		return "Position: " + str(self.position.get_x().round(2)) + ", " + str(self.position.get_y().round(2)) +
#			   " Velocity: " + str(self.velocity.get_x().round(2)) + ", " + str(self.velocity.get_y().round(2))
