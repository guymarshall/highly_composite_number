public class Main {
    public static void main(String[] args) {
        ResultPair resultPair = File.readResults();

        System.out.println("Starting values:");
        System.out.printf("Number: %d, Divisor count: %d%n", resultPair.lastNumber, resultPair.maxDivisors);

        //noinspection InfiniteLoopStatement
        while (true) {
            int divisors = Divisors.countDivisors(resultPair.lastNumber);
            if (divisors > resultPair.maxDivisors) {
                resultPair.maxDivisors = divisors;

                System.out.printf("Number: %d, Divisor count: %d%n", resultPair.lastNumber, resultPair.maxDivisors);
                File.writeResults(resultPair.lastNumber, resultPair.maxDivisors);
            }

            resultPair.lastNumber++;
        }
    }
}
