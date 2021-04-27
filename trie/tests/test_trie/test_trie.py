import unittest
from trie import SortedStringTrie


class TestTrie(unittest.TestCase):
    def setUp(self):
        self.words = 'an ant all allot alloy aloe are ate be'.split()
        # self.trie = SortedStringTrie(zip(self.words), range(len(self.words)))
        self.trie = SortedStringTrie()

    def test_trie(self):
        assert True