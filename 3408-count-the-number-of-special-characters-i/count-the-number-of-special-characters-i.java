class Solution {
    public int numberOfSpecialChars(String word) {
        int lowerMask = 0;
        int upperMask = 0;
        
        for (char c : word.toCharArray()) {
            if (c >= 'a' && c <= 'z') {
                lowerMask |= 1 << (c - 'a');
            } else {
                upperMask |= 1 << (c - 'A');
            }
        }
        
        int common = lowerMask & upperMask;
        
        return Integer.bitCount(common);
    }
}