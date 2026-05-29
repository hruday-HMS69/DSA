class Solution {

    private int digitSum(int n) {
        int sum = 0;
        while (n > 0) {
            sum += n % 10;
            n /= 10;
        }
        return sum;
    }

    public int minElement(int[] nums) {
        int min = Integer.MAX_VALUE;

        for (int num : nums) {
            int ds = digitSum(num);
            min = Math.min(min, ds);
        }

        return min;
    }
}