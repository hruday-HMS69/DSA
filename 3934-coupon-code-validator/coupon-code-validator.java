import java.util.*;

class Solution {
    public List<String> validateCoupons(String[] code, String[] businessLine, boolean[] isActive) {

        List<String> order = Arrays.asList(
            "electronics",
            "grocery",
            "pharmacy",
            "restaurant"
        );

        List<String[]> validCoupons = new ArrayList<>();

        int n = code.length;

        for (int i = 0; i < n; i++) {

            if (!isActive[i]) {
                continue;
            }

            if (!order.contains(businessLine[i])) {
                continue;
            }

            if (code[i].length() == 0) {
                continue;
            }

            if (!code[i].matches("[a-zA-Z0-9_]+")) {
                continue;
            }

            validCoupons.add(new String[]{businessLine[i], code[i]});
        }

        Collections.sort(validCoupons, (a, b) -> {

            int orderA = order.indexOf(a[0]);
            int orderB = order.indexOf(b[0]);

            if (orderA != orderB) {
                return orderA - orderB;
            }

            return a[1].compareTo(b[1]);
        });

        List<String> result = new ArrayList<>();

        for (String[] coupon : validCoupons) {
            result.add(coupon[1]);
        }

        return result;
    }
}