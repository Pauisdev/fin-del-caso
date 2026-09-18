extends Area2D
@export_multiline var text: String
var is_mouse_on_top = false


func _ready() -> void:
	connect("mouse_entered", on_mouse_entered)
	connect("mouse_exited", on_mouse_exited)

func on_mouse_entered():
	is_mouse_on_top = true
	
func on_mouse_exited():
	is_mouse_on_top = false

func _input(event: InputEvent) -> void:
	if not Input.is_action_just_pressed("click"):
		return
		
	if is_mouse_on_top and not %Ui.is_showing_text():
		%Ui.show_text(text)
