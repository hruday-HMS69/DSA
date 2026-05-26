import java.util.*;

class Solution {
    public int numberOfSpecialChars(String word) {

        Set<Character> lowerSet = new HashSet<>();
        Set<Character> upperSet = new HashSet<>();

        for (char ch : word.toCharArray()) {

            if (Character.isLowerCase(ch)) {
                lowerSet.add(ch);
            } else {
                upperSet.add(Character.toLowerCase(ch));
            }
        }

        int count = 0;

        for (char ch : lowerSet) {
            if (upperSet.contains(ch)) {
                count++;
            }
        }

        return count;
    }
}