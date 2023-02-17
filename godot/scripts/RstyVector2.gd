class_name RstyVector2

func distance(x1: float, y1: float, x2: float, y2: float) -> float:
	var a = x2 - x1
	var b = y2 - y1

	return sqrt(pow(a, 2) + pow(b, 2))

var x: float
var y: float

func _init(x: float, y: float) -> void:
	self.x = x
	self.y = y

func get_x() -> float:
	return self.x

func set_x(value: float) -> void:
	self.x = value

func get_y() -> float:
	return self.y

func set_y(value: float) -> void:
	self.y = value

func set_angle(angle: float) -> void:
	var length = self.get_length()
	self.x = cos(angle) * length
	self.y = sin(angle) * length

func get_angle() -> float:
	return atan2(self.y, self.x)

func get_length() -> float:
	return sqrt(pow(self.x, 2) + pow(self.y, 2))

func set_length(length: float) -> void:
	var angle = self.get_angle()
	self.x = cos(angle) * length
	self.y = sin(angle) * length

func add(v2: RstyVector2) -> RstyVector2:
	return RstyVector2.new(self.get_x() + v2.get_x(), self.get_y() + v2.get_y())

func subtract(v2: RstyVector2) -> RstyVector2:
	return RstyVector2.new(self.get_x() - v2.get_x(), self.get_y() - v2.get_y())

func multiply(scalar: float) -> RstyVector2:
	return RstyVector2.new(self.get_x() * scalar, self.get_y() * scalar)

func divide(scalar: float) -> RstyVector2:
	return RstyVector2.new(self.get_x() / scalar, self.get_y() / scalar)

func add_to(v2: RstyVector2) -> void:
	self.x += v2.get_x()
	self.y += v2.get_y()

func subtract_from(v2: RstyVector2) -> void:
	self.x -= v2.get_x()
	self.y -= v2.get_y()

func multiply_by(value: float) -> void:
	self.x *= value
	self.y *= value

func divide_by(value: float) -> void:
	self.x /= value
	self.y /= value
