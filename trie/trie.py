
import sys
from copy import copy 
from collections.abc import MutableMapping

# Singleton
class NULL:
    pass


class Node:
    """ Trie Node class
    """
    # improve performance for attribute lookup
    __slots__ = ('value', 'children')
    # A callable for creating a new attr:children mapping
    ChildrenFactory = dict

    def __init__(self, value=NULL):
        self.value = value
        self.children = self.ChildrenFactory()

    def __len__(self):
        # Return number of keys in subtree rooted at this node
        return int(self.value is not NULL) + sum(map(len, self.children.values()))

    def __repr__(self):
        return '(%s, {%s})' % (
            self.value is NULL and 'NULL' or repr(self.value),
            ', '.join('%r: %r' % t for t in self.children.items())
        )
    
    def __copy__(self):
        clone = self.__class__(self.value)
        clone_children = clone.children
        for key, child in self.children.items():
            clone_children[key] = child.__copy__()
        return clone 
    
    # This is for pickle purposes
    def __getstate__(self):
        return self.value, self.children
    
    def __setstate__(self, state):
        self.value, self.children = state



class Trie(MutableMapping):
    """Base trie class
    As with regular dicts, keys are not necessarily returned sorted
    Use SortedTrie if sorting is required
    """
    # Callable for forming a key from its parts
    KeyFactory = tuple
    # Callable for creating new trie nodes
    NodeFactory = Node

    def __init__(self, *args, **kwargs):
        """Create a new trie
        """
        self._root = self.NodeFactory()
        self.update(*args, **kwargs)

    @classmethod
    def fromkeys(cls, iterable, value=None):
        trie = cls()
        for key in iterable:
            trie[key] = value
        return trie

    #----- original mapping API methods ----------------------------------------
    



class StringTrie(Trie):
    KeyFactory = ''.join


class SortedStringTrie(StringTrie):
    """
     AAA
    """