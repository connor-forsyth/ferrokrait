from ferrokrait import *

# A simple node that derives the Node class.
class TestNode(Node):
    def _ready(self):  
        print("Hello,")
        
# Subclasses are usable too!
class TestNodeSubclass(TestNode):
    def _ready(self):
        print("World!")

# get the tree ( ), add a node ( TestNodeSubclass ), run the nodetree ( at a 60 fps cap )
get_tree().add_node(TestNodeSubclass).run(60)
