extends CPUParticles2D

func _input(event: InputEvent) -> void:
	if Input.is_action_just_pressed("click"):
		position = get_global_mouse_position()
		emitting = true
