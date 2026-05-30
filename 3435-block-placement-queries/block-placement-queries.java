import java.util.*;

class Solution {

    static class SegTree {
        int n;
        int[] tree;

        SegTree(int n) {
            this.n = n;
            tree = new int[4 * n];
        }

        void update(int idx, int val, int node, int l, int r) {
            if (l == r) {
                tree[node] = val;
                return;
            }
            int mid = (l + r) / 2;
            if (idx <= mid) update(idx, val, 2 * node, l, mid);
            else update(idx, val, 2 * node + 1, mid + 1, r);
            tree[node] = Math.max(tree[2 * node], tree[2 * node + 1]);
        }

        void update(int idx, int val) {
            update(idx, val, 1, 1, n);
        }

        int query(int ql, int qr, int node, int l, int r) {
            if (qr < l || r < ql) return 0;
            if (ql <= l && r <= qr) return tree[node];
            int mid = (l + r) / 2;
            return Math.max(
                query(ql, qr, 2 * node, l, mid),
                query(ql, qr, 2 * node + 1, mid + 1, r)
            );
        }

        int query(int l, int r) {
            if (l > r) return 0;
            return query(l, r, 1, 1, n);
        }
    }

    public List<Boolean> getResults(int[][] queries) {
        int MAX = 50000;

        TreeSet<Integer> obs = new TreeSet<>();
        obs.add(0);

        SegTree seg = new SegTree(MAX);

        List<Boolean> res = new ArrayList<>();

        for (int[] q : queries) {
            if (q[0] == 1) {
                int x = q[1];

                Integer left = obs.floor(x);
                Integer right = obs.ceiling(x);

                if (right == null) right = MAX;

                if (left != null && right != null) {
                    seg.update(right, 0);
                }

                seg.update(x, x - left);
                seg.update(right, right - x);

                obs.add(x);
            } else {
                int x = q[1];
                int sz = q[2];

                Integer left = obs.floor(x);
                if (left == null) left = 0;

                int best = seg.query(1, x);
                best = Math.max(best, x - left);

                res.add(best >= sz);
            }
        }

        return res;
    }
}