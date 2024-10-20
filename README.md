# ferrokrait - /ˈfɛroʊkreɪt/
A NodeTree based game development written in rust, for python. (severe WIP)
## Concept
ferrokrait provides several "Nodes" and builtin types which can be used in cohesion for agile and simple development, specifically tailored for games.
This module is heavily inspired by the game engine Godot, however does not use a user interface.

Here is an example of the current syntax:

```py
# tests/test.py
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
```
Output:
```
❯ python tests/test.py
Hello,
World!
```
How to support:
  - Test trial the module
  - Complain to me about bugs
  - Make suggestions
  - Contribute code
