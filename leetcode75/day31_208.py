class Trie:
    """
    root {}

    a {a: {}}

    p {a: {p: {}}}

    p {a: {p: {p: {}}}}

    l {a: {p: {p: {l: {}}}}}

    e {a: {p: {p: {l: {e: {}}}}}}

    $ {a: {p: {p: {l: {e: {$:$}}}}}}

    after insert app
    {a: {p: {p: {
                    l: {e: {$:$}},
                    $: $
    }}}}
    """

    def __init__(self):
        self.root = {}

    def insert(self, word: str) -> None:
        root = self.root
        for char in word:
            if char not in root:
                root[char] = {}
            root = root[char]
        root["$"] = "$"

    def search(self, word: str) -> bool:
        root = self.root
        for char in word:
            if char not in root:
                return False
            root = root[char]
        return root.get("$") == "$"

    def startsWith(self, prefix: str) -> bool:
        root = self.root
        for char in prefix:
            if char not in root:
                return False
            root = root[char]
        return True


if __name__ == "__main__":
    trie = Trie()
    trie.insert("apple")
    assert trie.search("apple") is True
    assert trie.search("app") is False
    assert trie.startsWith("app") is True
    trie.insert("app")
    assert trie.search("app") is True
