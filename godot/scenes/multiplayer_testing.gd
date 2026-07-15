extends Node2D
const PLAYER_CONTROLLER = preload("uid://bax3k0bosiqug")

var players: Array[CharacterBody2D]

func _ready() -> void:
	SteamNetworking.host_created.connect(on_host_created)
	
func on_host_created() -> void:
	spawn_player(multiplayer.get_unique_id())
	multiplayer.peer_connected.connect(spawn_player)
	
func spawn_player(peer_id: int) -> void:
	var new_player := PLAYER_CONTROLLER.instantiate()
	new_player.name = str(peer_id)
	add_child(new_player)
	initialize_player(new_player)

func initialize_player(player: CharacterBody2D) -> void:
	player.position = $Spawnpoint.position
	for other in players:
		player.add_collision_exception_with(other)
	players.append(player)


func _on_button_pressed() -> void:
	SteamNetworking.host_lobby()

# Only emmited on connected clients
func _on_multiplayer_spawner_spawned(node: Node) -> void:
	if node is CharacterBody2D:
		initialize_player(node)
