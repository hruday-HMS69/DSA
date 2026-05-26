class Solution {
    public int numberOfSpecialChars(String word) {
        int count = 0;

        for (char ch = 'a'; ch <= 'z'; ch++) {
            char lower = ch;
            char upper = Character.toUpperCase(ch);

            if (word.indexOf(lower) != -1 &&
                word.indexOf(upper) != -1) {
                count++;
            }
        }

        return count;
    }
}