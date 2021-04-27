from typing import Tuple 



class TrieNode(object):
    def __init__(self, char: str):
        self.char = char 
        self.children = []
        # Is it the last character of the word?
        self.word_finished = False
        # how many times this character appeared in the addition process
        self.counter = 1


def add(root, word: str):
    # Adding a word in the trie
    node = root
    for char in word:
        found_in_child = False 
        # Search for char in the children of current node
        for child in node.children:
            if child.char == char:
                child.counter += 1
                # Point the node to the child that contains this char
                node = child 
                found_in_child = True 
                break
        #
        if not found_in_child:
            new_node = TrieNode(char)
            node.children.append(new_node)
            node = new_node
    # Finished, mark it as end of a word
    node.word_finished = True



def find_prefix(root, prefix: str) -> Tuple[bool, int]:
    node = root
    if not node.children:
        return False, 0 
    for char in prefix:
        char_not_found = True
        for child in node.children:
            if child.char == char:
                char_not_found = False
                node = child 
                break
        if char_not_found:
            return False, 0
    return True, node.counter



if __name__ == "__main__":
    root = TrieNode("*")
    add(root, "hackathon")
    add(root, 'hack')

    print(find_prefix(root, 'hac'))
    print(find_prefix(root, 'hack'))
    print(find_prefix(root, 'hackathon'))
    print(find_prefix(root, 'ha'))
    print(find_prefix(root, 'hammer'))