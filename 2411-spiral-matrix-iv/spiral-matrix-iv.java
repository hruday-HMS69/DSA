/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public int[][] spiralMatrix(int m, int n, ListNode head) {
        int[][] ans = new int[m][n];
        for (var row : ans) {
            Arrays.fill(row, -1);
        }
        
        int i = 0, j = 0; // Initial position in the matrix
        // Directions represent right, down, left, and up movements
        final int[] dirs = {0, 1, 1, 0, 0, -1, -1, 0};
        int dirIndex = 0; // Index to determine current direction

        while (head != null) {
            ans[i][j] = head.val; // Assign value from the linked list
            head = head.next; // Move to the next node in the list
            
            // Calculate next position
            int nextI = i + dirs[dirIndex * 2];
            int nextJ = j + dirs[dirIndex * 2 + 1];

            // Check if the next position is valid and unvisited
            if (nextI >= 0 && nextI < m && nextJ >= 0 && nextJ < n && ans[nextI][nextJ] == -1) {
                // If valid, update position
                i = nextI;
                j = nextJ;
            } else {
                // If invalid, change direction (turn clockwise)
                dirIndex = (dirIndex + 1) % 4;
                // Update position to the new direction
                i += dirs[dirIndex * 2];
                j += dirs[dirIndex * 2 + 1];
            }
        }
        
        return ans; // Return filled matrix
    }
}
