extends Area2D
var is_mouse_on_top = false

func _process(delta: float) -> void:
	if Input.is_action_just_pressed("click") and is_mouse_on_top:
		$%Ui.show_text_box()

func _on_mouse_entered() -> void:
	is_mouse_on_top = true

func _on_mouse_exited() -> void:
	is_mouse_on_top = false
