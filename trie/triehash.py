class Trie(object):
    def __init__(self, words, compressed=False):
        def uncompressed_from_list(words):
            root = ({}, False)
            for word in words:
                node, is_terminal = root
                size = len(word)
                for i, char in enumerate(word):
                    is_terminal = i == size-1
                    if char not in node:
                        node[char] = ({}, is_terminal)
                    node, is_terminal = node[char]
            return root
        
        def compress_subtrie(root, prefix=''):
            '''
            any nodes that has only one child, gets concat with parent
            
            backtrack the prefix up the tree
            '''
            children, is_terminal = root
            if is_terminal:
                print("Terminal", root, prefix)
                return root, prefix
            if len(children) == 1:
                # Check further if subtrie rooted at this child can also be compressed
                # pass in current prefix to accumulate then backtrack the prefix up
                # not just the prefix, backtrack the node too
                for child_key, child_subtrie in children.items():
                    # feels like the traversal is weird
                    comp_child, comp_child_key = compress_subtrie(child_subtrie, prefix + child_key)
                    
                    return ({comp_child_key : comp_child}, is_terminal), comp_child_key 
            # for key, subtrie in children.items():


            return root, prefix

        root = uncompressed_from_list(words)
        if compressed:
            root, _ = compress_subtrie(root)
            print("Compress root", root)
        
        self.root = root
        self.is_compressed = compressed


    def print_trie(self):
        # For every children, print out its subtree until there's no more children
        # this is DFS
        def _print_subtrie(root, level):
            children, _ = root 
            for key, subtrie in children.items():
                is_terminal = subtrie[1]
                print(level * ("*" if is_terminal else "-"), key)
                _print_subtrie(subtrie, level + 1)
        _print_subtrie(self.root, 1)



if __name__ == "__main__":
    # trie = Trie(["hack", "hackathon", "high", "hitter", "actor", "hinters"])
    # trie.print_trie()

    trie = Trie(["hack"], True)
    trie.print_trie()