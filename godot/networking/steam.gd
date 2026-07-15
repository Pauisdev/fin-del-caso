extends Node
signal host_created

const LOBBY_TYPE := Steam.LobbyType.LOBBY_TYPE_FRIENDS_ONLY
const MAX_MEMBERS := 4
var peer: SteamMultiplayerPeer

func _ready() -> void:
	Steam.initRelayNetworkAccess()
	Steam.lobby_created.connect(on_lobby_created)
	Steam.lobby_joined.connect(on_lobby_joined)
	Steam.join_requested.connect(on_join_requested)
	
func _process(delta: float) -> void:
	Steam.run_callbacks()
	
func host_lobby() -> void:
	# Will emit "lobby_created" and "lobby_joined" signals
	Steam.createLobby(LOBBY_TYPE, MAX_MEMBERS)

# Only called on host's machine, after creating the lobby
func on_lobby_created(connect: int, lobby_id: int) -> void:
	if connect == Steam.RESULT_OK:
		peer = SteamMultiplayerPeer.new()
		peer.server_relay = true
		peer.create_host()
		multiplayer.multiplayer_peer = peer
		host_created.emit()
	
# Called when joining a lobby (everyone INCLUDING host since host directly joins their lobby after creating it)
func on_lobby_joined(lobby_id: int, permissions: int, locked: bool, response: int) -> void:
	if response == Steam.CHAT_ROOM_ENTER_RESPONSE_SUCCESS:
		# If we are the host, do not create a client_peer
		if Steam.getLobbyOwner(lobby_id) == Steam.getSteamID():
			return
		peer = SteamMultiplayerPeer.new()
		peer.server_relay = true
		peer.create_client(Steam.getLobbyOwner(lobby_id))
		multiplayer.multiplayer_peer = peer
		
# Called when someone attempts to join the lobby
func on_join_requested(lobby_id: int, steam_id: int) -> void:
	Steam.joinLobby(lobby_id)
