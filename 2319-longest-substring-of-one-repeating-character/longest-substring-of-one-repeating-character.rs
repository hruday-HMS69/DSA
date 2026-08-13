impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let chars: Vec<u8> = s.bytes().collect();
        let queries: Vec<u8> = query_characters.bytes().collect();

        let n = chars.len();

        let mut tree = vec![
            Node {
                left: 0,
                right: 0,
                prefix: 0,
                suffix: 0,
                best: 0,
                len: 0,
            };
            4 * n
        ];

        build(1, 0, n - 1, &chars, &mut tree);

        let mut ans = Vec::with_capacity(queries.len());

        for i in 0..queries.len() {
            let index = query_indices[i] as usize;

            update(
                1,
                0,
                n - 1,
                index,
                queries[i],
                &mut tree,
            );

            ans.push(tree[1].best as i32);
        }

        ans
    }
}

#[derive(Clone, Copy)]
struct Node {
    left: u8,
    right: u8,
    prefix: usize,
    suffix: usize,
    best: usize,
    len: usize,
}

fn build(
    node: usize,
    l: usize,
    r: usize,
    s: &[u8],
    tree: &mut [Node],
) {
    if l == r {
        tree[node] = Node {
            left: s[l],
            right: s[l],
            prefix: 1,
            suffix: 1,
            best: 1,
            len: 1,
        };
        return;
    }

    let mid = (l + r) / 2;

    build(node * 2, l, mid, s, tree);
    build(node * 2 + 1, mid + 1, r, s, tree);

    tree[node] = merge(tree[node * 2], tree[node * 2 + 1]);
}

fn update(
    node: usize,
    l: usize,
    r: usize,
    index: usize,
    ch: u8,
    tree: &mut [Node],
) {
    if l == r {
        tree[node] = Node {
            left: ch,
            right: ch,
            prefix: 1,
            suffix: 1,
            best: 1,
            len: 1,
        };
        return;
    }

    let mid = (l + r) / 2;

    if index <= mid {
        update(node * 2, l, mid, index, ch, tree);
    } else {
        update(node * 2 + 1, mid + 1, r, index, ch, tree);
    }

    tree[node] = merge(tree[node * 2], tree[node * 2 + 1]);
}

fn merge(a: Node, b: Node) -> Node {
    let len = a.len + b.len;

    let mut prefix = a.prefix;
    let mut suffix = b.suffix;

    let mut best = a.best.max(b.best);

    if a.right == b.left {
        if a.prefix == a.len {
            prefix = a.len + b.prefix;
        }

        if b.suffix == b.len {
            suffix = b.len + a.suffix;
        }

        best = best.max(a.suffix + b.prefix);
    }

    Node {
        left: a.left,
        right: b.right,
        prefix,
        suffix,
        best,
        len,
    }
}