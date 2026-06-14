extends CPUParticles2D

func _input(event):
	if event is InputEventMouseButton:
		if event.pressed:
			position = event.position
			emitting = true
