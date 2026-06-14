extends TextureRect
@export var text_speed = 20

var text_to_show = null
var letters_to_show = 0
var ignored_first_click = false

func _process(delta: float) -> void:
	if text_to_show == null: 
		return
	letters_to_show += delta * text_speed
	$RichTextLabel.text = text_to_show.left(round(letters_to_show))
	
	if Input.is_action_just_pressed("click"):
		if not ignored_first_click: 
			ignored_first_click = true
			return
			
		if letters_to_show < text_to_show.length():
			letters_to_show = text_to_show.length()
		else:
			visible = false

func show_text(text: String):
	if visible: return
	visible = true
	text_to_show = text
	letters_to_show = 0
	ignored_first_click = false
