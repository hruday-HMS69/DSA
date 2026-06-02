class Solution {
    public int earliestFinishTime(int[] landStartTime, int[] landDuration,
                                   int[] waterStartTime, int[] waterDuration) {
        int n = landStartTime.length;
        int m = waterStartTime.length;
        int minFinish = Integer.MAX_VALUE;

        for (int i = 0; i < n; i++) {
            int landStart = landStartTime[i];
            int landDur = landDuration[i];

            for (int j = 0; j < m; j++) {
                int waterStart = waterStartTime[j];
                int waterDur = waterDuration[j];

                int finishLandFirst = Math.max(landStart + landDur, waterStart) + waterDur;

                int finishWaterFirst = Math.max(waterStart + waterDur, landStart) + landDur;

                int bestForPair = Math.min(finishLandFirst, finishWaterFirst);
                minFinish = Math.min(minFinish, bestForPair);
            }
        }
        return minFinish;
    }
}