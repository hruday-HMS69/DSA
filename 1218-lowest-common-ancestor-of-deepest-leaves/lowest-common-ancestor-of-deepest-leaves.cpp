/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    TreeNode* lcaDeepestLeaves(TreeNode* root) {
        vector<TreeNode*> leaves;
        findLeaves(root, leaves);
        
        int maxDepth = findMaxDepth(root);
        
        vector<TreeNode*> deepestLeaves;
        for (TreeNode* leaf : leaves) {
            if (getDepth(root, leaf) == maxDepth) {
                deepestLeaves.push_back(leaf);
            }
        }
        
        TreeNode* lca = deepestLeaves[0];
        for (int i = 1; i < deepestLeaves.size(); ++i) {
            lca = findLCA(root, lca, deepestLeaves[i]);
        }
        
        return lca;
    }
    
private:
    void findLeaves(TreeNode* root, vector<TreeNode*>& leaves) {
        if (!root) return;
        if (!root->left && !root->right) {
            leaves.push_back(root);
        }
        findLeaves(root->left, leaves);
        findLeaves(root->right, leaves);
    }
    
    int findMaxDepth(TreeNode* root) {
        if (!root) return 0;
        return 1 + max(findMaxDepth(root->left), findMaxDepth(root->right));
    }
    
    int getDepth(TreeNode* root, TreeNode* node) {
        if (!root) return 0;
        if (root == node) return 1;
        int leftDepth = getDepth(root->left, node);
        if (leftDepth > 0) return leftDepth + 1;
        int rightDepth = getDepth(root->right, node);
        return rightDepth > 0 ? rightDepth + 1 : 0;
    }

    TreeNode* findLCA(TreeNode* root, TreeNode* p, TreeNode* q) {
        if (!root || root == p || root == q) return root;
        TreeNode* left = findLCA(root->left, p, q);
        TreeNode* right = findLCA(root->right, p, q);
        if (left && right) return root;
        return left ? left : right;
    }
};
