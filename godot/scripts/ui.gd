extends CanvasLayer

func show_text_box():
	$TextBox.visible = true

func hide_text_box():
	$TextBox.visible = false

func show_text(text: String):
	$TextBox.show_text(text)
