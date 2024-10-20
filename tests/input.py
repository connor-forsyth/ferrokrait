from ferrokrait import *

class InputTest(Node):
    def _on_key_input(self):
        print("guau")
        if Input.get_key(Key.Q) and Input.get_key(Key.LControl):
            print("exit :)")
    def _process(self, delta):
        print("")

get_tree().add_node(InputTest).run(60)