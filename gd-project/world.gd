extends Node2D

var icon = preload("res://icon.tscn")

var _click_callback = JavaScriptBridge.create_callback(_on_click)

var rng = RandomNumberGenerator.new()


# Called when the node enters the scene tree for the first time.
func _ready():	
	JavaScriptBridge.get_interface("window").addEventListener("_on_click", _click_callback)
	print("Ready, printing to browser console")


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func _on_click(args: Array):
	# Callback when event is fired
	print(args[0].detail)
	var instance = icon.instantiate()
	var x = rng.randi_range(50, 750)
	var y = rng.randi_range(50, 550)
	
	add_child(instance)
	instance.position = Vector2(x, y)
	
