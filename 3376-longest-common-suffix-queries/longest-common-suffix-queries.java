class Solution {

    class TrieNode {
        TrieNode[] children = new TrieNode[26];

        int idx = -1;
    }

    TrieNode root = new TrieNode();
    String[] wordsContainer;

    public int[] stringIndices(String[] wordsContainer, String[] wordsQuery) {
        this.wordsContainer = wordsContainer;

        for (int i = 0; i < wordsContainer.length; i++) {
            insert(wordsContainer[i], i);
        }

        int[] ans = new int[wordsQuery.length];

        for (int i = 0; i < wordsQuery.length; i++) {
            ans[i] = query(wordsQuery[i]);
        }

        return ans;
    }

    private void insert(String word, int index) {
        TrieNode node = root;

        updateBest(node, index);

        for (int i = word.length() - 1; i >= 0; i--) {
            int c = word.charAt(i) - 'a';

            if (node.children[c] == null) {
                node.children[c] = new TrieNode();
            }

            node = node.children[c];

            updateBest(node, index);
        }
    }

    private void updateBest(TrieNode node, int index) {
        if (node.idx == -1) {
            node.idx = index;
            return;
        }

        int oldIdx = node.idx;

        int oldLen = wordsContainer[oldIdx].length();
        int newLen = wordsContainer[index].length();

        if (newLen < oldLen || (newLen == oldLen && index < oldIdx)) {
            node.idx = index;
        }
    }

    private int query(String word) {
        TrieNode node = root;

        for (int i = word.length() - 1; i >= 0; i--) {
            int c = word.charAt(i) - 'a';

            if (node.children[c] == null) {
                break;
            }

            node = node.children[c];
        }

        return node.idx;
    }
}