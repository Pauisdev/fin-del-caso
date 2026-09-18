extends TextureRect

@export var text_speed: int
var characters_to_show = 0
const SPECIAL_TEXTS = {
	SKIP = "_SKIP"
}

func _process(delta: float) -> void:
	characters_to_show += delta * text_speed
	$Text.visible_characters = characters_to_show
	
func show_text(text: String) -> void:
	if text == SPECIAL_TEXTS.SKIP: 
		if characters_to_show == 0: return
		if characters_to_show >= len($Text.text):
			get_parent().show_non_speaking_ui()
		else:
			characters_to_show = len($Text.text)
		return
	characters_to_show = 0
	$Text.text = text
	
func _input(event: InputEvent) -> void:
	if Input.is_action_just_pressed("click") and visible: # If any text is already showing, skip it
		skip_text()
		
func skip_text() -> void:
	show_text(SPECIAL_TEXTS.SKIP)
