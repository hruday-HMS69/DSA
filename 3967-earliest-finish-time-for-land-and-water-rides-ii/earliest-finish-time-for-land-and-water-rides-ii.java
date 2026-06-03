import java.util.Arrays;

class Solution {
    public int earliestFinishTime(int[] landStartTime, int[] landDuration,
                                   int[] waterStartTime, int[] waterDuration) {
        return Math.min(
            solve(landStartTime, landDuration, waterStartTime, waterDuration),
            solve(waterStartTime, waterDuration, landStartTime, landDuration)
        );
    }

    private int solve(int[] firstStart, int[] firstDur,
                      int[] secondStart, int[] secondDur) {
        int m = secondStart.length;

        int[][] second = new int[m][2];
        for (int i = 0; i < m; i++) {
            second[i][0] = secondStart[i];
            second[i][1] = secondDur[i];
        }

        Arrays.sort(second, (a, b) -> Integer.compare(a[0], b[0]));

        int[] minDur = new int[m];
        minDur[0] = second[0][1];
        for (int i = 1; i < m; i++) {
            minDur[i] = Math.min(minDur[i - 1], second[i][1]);
        }

        int[] minSum = new int[m];
        minSum[m - 1] = second[m - 1][0] + second[m - 1][1];
        for (int i = m - 2; i >= 0; i--) {
            minSum[i] = Math.min(minSum[i + 1], second[i][0] + second[i][1]);
        }

        int best = Integer.MAX_VALUE;

        for (int i = 0; i < firstStart.length; i++) {
            int end = firstStart[i] + firstDur[i];

            int idx = binarySearch(second, end);

            if (idx >= 0) {
                best = Math.min(best, end + minDur[idx]);
            }
            if (idx + 1 < m) {
                best = Math.min(best, minSum[idx + 1]);
            }
        }

        return best;
    }

    private int binarySearch(int[][] arr, int target) {
        int left = 0, right = arr.length - 1;
        int result = -1;

        while (left <= right) {
            int mid = left + (right - left) / 2;

            if (arr[mid][0] <= target) {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        return result;
    }
}