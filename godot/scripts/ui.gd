extends CanvasLayer

func show_speaking_ui():
	$TextBox.show()
	$Shadow.show()
	$Character.show()
	$Badge.show()
	$Inventory.hide()

func show_non_speaking_ui():
	$TextBox.hide()
	$Shadow.hide()
	$Character.hide()
	$Badge.hide()
	$Inventory.show()

func show_text(text: String) -> void:
	show_speaking_ui()
	$TextBox.show_text(text)

func is_showing_text() -> bool:
	return $TextBox.visible
