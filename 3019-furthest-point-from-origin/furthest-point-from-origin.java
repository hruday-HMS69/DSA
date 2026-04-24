class Solution {
    public int furthestDistanceFromOrigin(String moves) {
        int balance = 0, blank = 0;

        for (char c : moves.toCharArray()) {
            if (c == 'L') balance--;
            else if (c == 'R') balance++;
            else blank++;
        }

        return Math.abs(balance) + blank;
    }
}