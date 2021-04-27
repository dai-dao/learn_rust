import unittest
from trie import Node


class TestNode(unittest.TestCase):

    def setUp(self):
        self.node = Node(1)

    def test_repr(self):
        evaled = eval(repr(self.node))
        self.assertEqual(repr(evaled), repr(self.node))