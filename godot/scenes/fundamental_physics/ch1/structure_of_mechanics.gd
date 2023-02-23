extends Node


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# y(t) = (1/2 * (a * t ^ 2)) + (b * t) + c
# t = time in seconds
# a = acceleration
# b = Initial velocity
# c = Initial position
# Returns the position of an object at time t.
func eq_one_point_six(t: float, a: float, b: float, c: float):
	return (1/2 * (a  * pow(t, 2))) + (b * t) + c


#problem 3
#Romeo is at x = 0 m at t = 0 s when he sees Juliet at x = 6 m.



# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
