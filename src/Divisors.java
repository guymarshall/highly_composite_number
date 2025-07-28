public class Divisors {
    public static int countDivisors(int number) {
        int divisor_count = 1;
        int current_divisor = 2;
        while (current_divisor * current_divisor <= number) {
            int exponent = 0;
            while (number % current_divisor == 0) {
                number /= current_divisor;
                exponent++;
            }
            if (exponent > 0) {
                divisor_count *= exponent + 1;
            }
            current_divisor++;
        }
        if (number > 1) {
            divisor_count *= 2;
        }
        return divisor_count;
    }
}
